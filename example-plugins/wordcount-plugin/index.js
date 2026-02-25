// 字数统计插件
console.log('[字数统计插件] 已加载');

class WordCounter {
  constructor() {
    this.name = '字数统计工具';
  }

  countWords(text) {
    if (!text) {
      return { words: 0, chars: 0, charsNoSpaces: 0, paragraphs: 0, sentences: 0 };
    }

    const trimmedText = text.trim();
    
    if (trimmedText === '') {
      return { words: 0, chars: 0, charsNoSpaces: 0, paragraphs: 0, sentences: 0 };
    }

    const chars = text.length;
    const charsNoSpaces = text.replace(/\s/g, '').length;
    
    const paragraphs = trimmedText.split(/\n\s*\n/).filter(p => p.trim()).length;
    
    const sentences = trimmedText.split(/[。！？.!?]+/).filter(s => s.trim()).length;
    
    const words = trimmedText.split(/[\s\u2000-\u206F\u2E00-\u2E7F\\'!"#$%&()*+,\-.\/:;<=>?@\[\]^`{|}~]+/)
      .filter(w => w.trim()).length;

    return {
      words,
      chars,
      charsNoSpaces,
      paragraphs,
      sentences
    };
  }

  formatCount(count) {
    return count.toLocaleString('zh-CN');
  }

  count(text) {
    const result = this.countWords(text);
    console.log('[字数统计] 统计结果:', result);
    return result;
  }
}

if (typeof window !== 'undefined') {
  window.wordcountPlugin = new WordCounter();
  console.log('[字数统计插件] 插件已就绪，可通过 window.wordcountPlugin 访问');
}
