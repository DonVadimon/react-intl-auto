// 测试正则表达式的行为
const path = 'src/components/App.js';
const regex = /src.*/;

console.log('Original path:', path);
console.log('Regex pattern:', regex);
console.log('Match result:', path.match(regex));
console.log('Replace result:', path.replace(regex, ''));

// 测试更合适的正则表达式
const regex2 = /src\/.*\//;
console.log('\nBetter regex pattern:', regex2);
console.log('Match result:', path.match(regex2));
console.log('Replace result:', path.replace(regex2, ''));
