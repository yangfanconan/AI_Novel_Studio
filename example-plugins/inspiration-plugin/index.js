// 灵感生成器插件
console.log('[灵感生成器] 已加载');

class InspirationGenerator {
  constructor() {
    this.name = '灵感生成器';
  }

  getGenericInspirations() {
    return [
      '一个隐藏在图书馆里的古老秘密',
      '突然发现自己能听懂动物的语言',
      '收到一封来自未来的信',
      '发现一个能回到过去的怀表',
      '在废弃的游乐园里遇到了不可思议的朋友',
      '镜子里的倒影开始有了自己的意识',
      '一本会自动书写的日记',
      '一条通往异世界的神秘隧道',
      '一个永不枯萎的花园',
      '能够看到人们过去的回忆',
      '城市上空出现了一座漂浮的岛屿',
      '每到午夜，时间会倒流一小时',
      '发现了一个能实现三个愿望的神灯',
      '一只会说话的猫声称自己是巫师',
      '打开门后进入了另一个平行世界',
      '在梦中预见了未来的事件',
      '发现了一个记录着所有人命运的古老书卷',
      '身体里沉睡着另一个意识',
      '获得了读取别人内心想法的能力',
      '每天都会收到来自未知号码的预言短信',
      '家里的物品开始有了生命'
    ];
  }

  getCharacterInspirations() {
    return [
      '一个沉默寡言但内心丰富的图书管理员',
      '一个失去记忆的前特种兵',
      '一个能看见鬼魂的少年',
      '一个在时间裂缝中穿梭的旅人',
      '一个拥有治愈之力的医生',
      '一个被诅咒永远无法说谎的商人',
      '一个半人半机器的赏金猎人',
      '一个能操控植物的女巫',
      '一个隐藏身份的逃亡王子',
      '一个在街头流浪的天才黑客',
      '一个能预知死亡但无法改变的先知',
      '一个为了寻找真相而踏上旅程的侦探',
      '一个背负着家族诅咒的继承人',
      '一个来自异世界的失忆少女',
      '一个能看见别人过去的眼镜少年',
      '一个想要成为英雄的反派'
    ];
  }

  getPlotInspirations() {
    return [
      '主角发现了一个能改变过去的机会，但代价是失去现在最珍贵的人',
      '两个对立的阵营中，两个相爱的人被迫站在对立面',
      '主角必须在自己和拯救世界之间做出选择',
      '一个看似普通的任务，却意外揭开了一个巨大的阴谋',
      '主角被误认为是预言中的救世主，被迫承担起责任',
      '一个看似完美的世界，背后隐藏着黑暗的秘密',
      '主角必须学会控制自己的能力，否则会摧毁所爱的一切',
      '为了寻找失踪的亲人，主角踏上了一段危险的旅程',
      '主角发现自己的导师其实是敌人',
      '一个简单的误会引发了连锁反应，最终导致了战争',
      '主角必须在有限的时间内完成不可能的任务',
      '一个看似微小的决定，改变了整个故事的走向',
      '主角发现自己的过去并非如记忆中那样',
      '两个世界即将碰撞，主角必须阻止这场灾难',
      '主角发现自己拥有强大的力量，但不知道如何使用',
      '一个神秘的陌生人改变了主角的命运'
    ];
  }

  generate(type = 'generic') {
    let inspirations;
    
    switch (type) {
      case 'character':
        inspirations = this.getCharacterInspirations();
        break;
      case 'plot':
        inspirations = this.getPlotInspirations();
        break;
      default:
        inspirations = this.getGenericInspirations();
    }

    const randomIndex = Math.floor(Math.random() * inspirations.length);
    const inspiration = inspirations[randomIndex];
    
    console.log(`[灵感生成器] 生成${type}灵感:`, inspiration);
    return inspiration;
  }

  generateMultiple(type = 'generic', count = 5) {
    const results = [];
    for (let i = 0; i < count; i++) {
      results.push(this.generate(type));
    }
    return results;
  }

  getAll(type) {
    switch (type) {
      case 'character':
        return this.getCharacterInspirations();
      case 'plot':
        return this.getPlotInspirations();
      default:
        return this.getGenericInspirations();
    }
  }
}

if (typeof window !== 'undefined') {
  window.inspirationPlugin = new InspirationGenerator();
  console.log('[灵感生成器] 插件已就绪，可通过 window.inspirationPlugin 访问');
  
  // 打印使用示例
  console.log('[灵感生成器] 使用示例:');
  console.log('  window.inspirationPlugin.generate()          - 生成通用灵感');
  console.log('  window.inspirationPlugin.generate("character") - 生成角色灵感');
  console.log('  window.inspirationPlugin.generate("plot")     - 生成情节灵感');
  console.log('  window.inspirationPlugin.generateMultiple()    - 生成多个灵感');
  console.log('  window.inspirationPlugin.getAll()           - 获取所有灵感列表');
}
