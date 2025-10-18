// 模拟 dot_path 函数
function dot_path(str, separator) {
    return str.replace(/\//g, separator);
}

// 测试
const filename = 'src/components/App.js';
const relative_to = 'src';
const separator = '.';

// 移除文件扩展名
let base_path = filename.replace(/\.[^/.]+$/, '');
console.log('Base path after removing extension:', base_path);

// 转换为点分隔符
base_path = dot_path(base_path, separator);
console.log('Base path after dot_path:', base_path);

// 转换 relative_to
const relative_to_dots = dot_path(relative_to, separator);
console.log('Relative_to after dot_path:', relative_to_dots);

if (base_path.startsWith(relative_to_dots)) {
    let remaining = base_path.substring(relative_to_dots.length);
    console.log('Remaining after removing relative_to:', remaining);
    
    if (remaining.startsWith(separator)) {
        remaining = remaining.substring(separator.length);
        console.log('Final result after removing leading separator:', remaining);
    } else {
        console.log('Final result (no leading separator to remove):', remaining);
    }
} else {
    console.log('Base path does not start with relative_to');
}
