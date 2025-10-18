// 调试项目根目录检测
const fs = require('fs');
const path = require('path');

function findProjectRoot(filePath) {
  let current = path.dirname(filePath);
  
  const projectIndicators = [
    'yarn.lock',
    'package.json',
    'package-lock.json',
    'tsconfig.json',
    'babel.config.js',
    'webpack.config.js',
    '.git',
  ];
  
  while (true) {
    for (const indicator of projectIndicators) {
      if (fs.existsSync(path.join(current, indicator))) {
        return current;
      }
    }
    
    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }
  
  return null;
}

const filename = '/Users/ryan/project/src/components/App.js';
console.log('Filename:', filename);
console.log('Project root:', findProjectRoot(filename));
console.log('Expected project root: /Users/ryan/project');
