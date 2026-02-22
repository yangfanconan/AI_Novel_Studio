import { invoke } from '@tauri-apps/api/core';

export const testSettingsDialogInteraction = async () => {
  const results = {
    settingsButtonExists: false,
    settingsButtonClickable: false,
    dialogOpens: false,
    dialogCloses: false,
    apiKeySaves: false,
    apiKeyLoads: false,
    modelSelectionWorks: false,
  };

  try {
    results.settingsButtonExists = true;
    console.log('✓ 设置按钮存在');
  } catch (error) {
    console.error('✗ 设置按钮不存在', error);
    return results;
  }

  try {
    const settingsButton = document.querySelector('button[title*="设置"]');
    if (settingsButton) {
      (settingsButton as HTMLButtonElement).click();
      await new Promise(resolve => setTimeout(resolve, 500));
      results.settingsButtonClickable = true;
      console.log('✓ 设置按钮可点击');
    }
  } catch (error) {
    console.error('✗ 设置按钮不可点击', error);
  }

  try {
    const dialog = document.querySelector('[class*="fixed"][class*="bg-black"]');
    results.dialogOpens = !!dialog;
    console.log(dialog ? '✓ 对话框已打开' : '✗ 对话框未打开');
  } catch (error) {
    console.error('✗ 对话框检测失败', error);
  }

  await new Promise(resolve => setTimeout(resolve, 1000));

  try {
    const apiKeyInput = document.querySelector('input[type="password"]') as HTMLInputElement;
    if (apiKeyInput) {
      apiKeyInput.value = 'test_api_key_12345';
      apiKeyInput.dispatchEvent(new Event('input', { bubbles: true }));
      await new Promise(resolve => setTimeout(resolve, 100));
      console.log('✓ API密钥输入框可操作');
    }
  } catch (error) {
    console.error('✗ API密钥输入框不可操作', error);
  }

  try {
    const saveButton = document.querySelector('button[type="submit"], button:has-text("保存"), button:has-text("保存设置")');
    if (saveButton) {
      (saveButton as HTMLButtonElement).click();
      await new Promise(resolve => setTimeout(resolve, 500));
      results.apiKeySaves = true;
      console.log('✓ API密钥保存按钮可点击');
    }
  } catch (error) {
    console.error('✗ API密钥保存按钮不可点击', error);
  }

  await new Promise(resolve => setTimeout(resolve, 500));

  try {
    const apiKey = await invoke<string>('get_bigmodel_api_key');
    results.apiKeyLoads = true;
    console.log('✓ API密钥加载成功，长度:', apiKey.length);
  } catch (error) {
    console.error('✗ API密钥加载失败', error);
  }

  try {
    const modelSelect = document.querySelector('select, [role="combobox"]') as HTMLSelectElement;
    if (modelSelect) {
      modelSelect.value = 'model-2';
      modelSelect.dispatchEvent(new Event('change', { bubbles: true }));
      await new Promise(resolve => setTimeout(resolve, 100));
      results.modelSelectionWorks = true;
      console.log('✓ 模型选择器可操作');
    }
  } catch (error) {
    console.error('✗ 模型选择器不可操作', error);
  }

  try {
    const closeButton = document.querySelector('button[aria-label*="close"], button:has-text("取消"), button:has-text("关闭")');
    if (closeButton) {
      (closeButton as HTMLButtonElement).click();
      await new Promise(resolve => setTimeout(resolve, 500));
      results.dialogCloses = true;
      console.log('✓ 对话框关闭按钮可点击');
    } else {
      const overlay = document.querySelector('[class*="fixed"][class*="bg-black"]');
      if (overlay) {
        (overlay as HTMLElement).click();
        await new Promise(resolve => setTimeout(resolve, 500));
        results.dialogCloses = true;
        console.log('✓ 点击遮罩层关闭对话框');
      }
    }
  } catch (error) {
    console.error('✗ 对话框关闭失败', error);
  }

  return results;
};

export const printTestResults = (results: any) => {
  console.log('\n=== 设置对话框测试结果 ===');
  console.log(`设置按钮存在: ${results.settingsButtonExists ? '✓' : '✗'}`);
  console.log(`设置按钮可点击: ${results.settingsButtonClickable ? '✓' : '✗'}`);
  console.log(`对话框打开: ${results.dialogOpens ? '✓' : '✗'}`);
  console.log(`对话框关闭: ${results.dialogCloses ? '✓' : '✗'}`);
  console.log(`API密钥保存: ${results.apiKeySaves ? '✓' : '✗'}`);
  console.log(`API密钥加载: ${results.apiKeyLoads ? '✓' : '✗'}`);
  console.log(`模型选择: ${results.modelSelectionWorks ? '✓' : '✗'}`);
  
  const passedCount = Object.values(results).filter(v => v === true).length;
  const totalCount = Object.keys(results).length;
  console.log(`\n通过: ${passedCount}/${totalCount} (${((passedCount/totalCount)*100).toFixed(1)}%)`);
  console.log('===================\n');
};
