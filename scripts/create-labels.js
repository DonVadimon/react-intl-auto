#!/usr/bin/env node

const https = require('https');

// GitHub API configuration
const GITHUB_TOKEN = process.env.GITHUB_TOKEN;
const REPO_OWNER = 'lcl9288';
const REPO_NAME = 'swc-plugin-react-intl-auto';

if (!GITHUB_TOKEN) {
  console.error('❌ GITHUB_TOKEN environment variable is required');
  console.log('Usage: GITHUB_TOKEN=your_token node scripts/create-labels.js');
  process.exit(1);
}

// Labels to create
const labels = [
  { name: 'dependencies', color: '0366d6', description: 'Dependency updates' },
  { name: 'javascript', color: 'f1e05a', description: 'JavaScript related' },
  { name: 'rust', color: 'dea584', description: 'Rust related' },
  { name: 'automated', color: '7057ff', description: 'Automated changes' },
  { name: 'ci', color: 'f9d0c4', description: 'Continuous Integration' },
  { name: 'build', color: 'f9d0c4', description: 'Build system' },
  { name: 'test', color: 'f9d0c4', description: 'Testing' },
  { name: 'chore', color: 'f9d0c4', description: 'Maintenance tasks' },
  { name: 'feat', color: 'a2eeef', description: 'New features' },
  { name: 'fix', color: 'd73a4a', description: 'Bug fixes' },
  { name: 'refactor', color: 'a2eeef', description: 'Code refactoring' },
  { name: 'perf', color: 'a2eeef', description: 'Performance improvements' },
  { name: 'style', color: 'a2eeef', description: 'Code style changes' },
  { name: 'release', color: '7057ff', description: 'Release related' }
];

function makeRequest(method, path, data = null) {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'api.github.com',
      port: 443,
      path: path,
      method: method,
      headers: {
        'Authorization': `token ${GITHUB_TOKEN}`,
        'User-Agent': 'create-labels-script/1.0.0',
        'Accept': 'application/vnd.github.v3+json',
        'Content-Type': 'application/json'
      }
    };

    const req = https.request(options, (res) => {
      let responseData = '';
      res.on('data', (chunk) => responseData += chunk);
      res.on('end', () => {
        try {
          const result = JSON.parse(responseData);
          resolve({ status: res.statusCode, data: result });
        } catch (e) {
          resolve({ status: res.statusCode, data: responseData });
        }
      });
    });

    req.on('error', reject);

    if (data) {
      req.write(JSON.stringify(data));
    }

    req.end();
  });
}

async function getExistingLabels() {
  try {
    const response = await makeRequest('GET', `/repos/${REPO_OWNER}/${REPO_NAME}/labels`);
    if (response.status === 200) {
      return response.data.map(label => label.name);
    }
    return [];
  } catch (error) {
    console.error('Error fetching existing labels:', error.message);
    return [];
  }
}

async function createLabel(label) {
  try {
    const response = await makeRequest('POST', `/repos/${REPO_OWNER}/${REPO_NAME}/labels`, label);
    
    if (response.status === 201) {
      console.log(`✅ Created label: ${label.name}`);
      return true;
    } else if (response.status === 422) {
      console.log(`⚠️ Label '${label.name}' already exists`);
      return true;
    } else {
      console.error(`❌ Failed to create label '${label.name}':`, response.data);
      return false;
    }
  } catch (error) {
    console.error(`❌ Error creating label '${label.name}':`, error.message);
    return false;
  }
}

async function main() {
  console.log('🏷️ Creating GitHub repository labels...');
  console.log(`Repository: ${REPO_OWNER}/${REPO_NAME}`);
  console.log('');

  // Get existing labels
  const existingLabels = await getExistingLabels();
  console.log(`Found ${existingLabels.length} existing labels`);

  let created = 0;
  let skipped = 0;
  let failed = 0;

  for (const label of labels) {
    if (existingLabels.includes(label.name)) {
      console.log(`⏭️ Skipping existing label: ${label.name}`);
      skipped++;
    } else {
      const success = await createLabel(label);
      if (success) {
        created++;
      } else {
        failed++;
      }
    }
  }

  console.log('');
  console.log('📊 Summary:');
  console.log(`✅ Created: ${created}`);
  console.log(`⏭️ Skipped: ${skipped}`);
  console.log(`❌ Failed: ${failed}`);

  if (failed === 0) {
    console.log('');
    console.log('🎉 All labels processed successfully!');
    console.log('Dependabot should now work without label errors.');
  } else {
    console.log('');
    console.log('⚠️ Some labels failed to create. Please check the errors above.');
    process.exit(1);
  }
}

main().catch(console.error);
