// 调试 relative_to 处理
const path = require('path');

// 模拟我们的逻辑
function debugRelativeTo(relativeTo, filename) {
  console.log('Input:');
  console.log('  relativeTo:', relativeTo);
  console.log('  filename:', filename);
  console.log('  current dir:', process.cwd());
  
  // 检查 relativeTo 是否是绝对路径
  const isAbsolute = path.isAbsolute(relativeTo);
  console.log('  relativeTo is absolute:', isAbsolute);
  
  // 计算 relativeTo 的绝对路径
  const relativeToPath = isAbsolute ? relativeTo : path.join(process.cwd(), relativeTo);
  console.log('  relativeToPath:', relativeToPath);
  
  // 计算 filename 的绝对路径
  const filenamePath = path.isAbsolute(filename) ? filename : path.join(process.cwd(), filename);
  console.log('  filenamePath:', filenamePath);
  
  // 计算相对路径
  try {
    const relativePath = path.relative(relativeToPath, filenamePath);
    console.log('  relativePath:', relativePath);
    
    // 移除文件扩展名
    const withoutExt = path.parse(relativePath).name;
    console.log('  withoutExt:', withoutExt);
    
    // 转换为点分隔符
    const dotPath = relativePath.replace(/[\/\\]/g, '.');
    console.log('  dotPath:', dotPath);
    
    return dotPath;
  } catch (error) {
    console.log('  Error:', error.message);
    return null;
  }
}

console.log('Test 1: relative_to: "src"');
debugRelativeTo('src', '/Users/ryan/project/src/components/App.js');

console.log('\nTest 2: relative_to: "/Users/ryan/project/src"');
debugRelativeTo('/Users/ryan/project/src', '/Users/ryan/project/src/components/App.js');
