// This script extracts class names from a Rust file and generates a Tailwind CSS safelist.
// Make sure to adjust the path to your Rust file as needed.
// You can run this script with Node.js to generate the safelist file.
// Usage: node generate_tailwind_safelist.js
const fs = require("fs");
const path = require("path");

function collectRsFiles(dir, files = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      collectRsFiles(fullPath, files);
    } else if (entry.isFile() && entry.name.endsWith(".rs")) {
      files.push(fullPath);
    }
  }
  return files;
}

function extractClassesFromContent(content) {
  const classes = new Set();

  // Patterns to catch different Tailwind class usages
  const patterns = [
    /class\s*=\s*"([^"]+)"/g,          // HTML style: class="..."
    /classes!\(([^)]+)\)/g,            // classes!(...)
    /"([a-z0-9:_/-]+)"/gi              // fallback: any quoted word (cautious)
  ];

  for (const regex of patterns) {
    let match;
    while ((match = regex.exec(content)) !== null) {
      const segment = match[1];
      if (!segment) continue;
      segment
        .replace(/["',]/g, " ")
        .split(/\s+/)
        .filter(cls => cls.trim().length > 0)
        .forEach(cls => classes.add(cls));
    }
  }

  return classes;
}

function extractClassesFromDir(dir) {
  const rsFiles = collectRsFiles(dir);
  const allClasses = new Set();

  for (const file of rsFiles) {
    const content = fs.readFileSync(file, "utf8");
    extractClassesFromContent(content).forEach(cls => allClasses.add(cls));
  }

  return Array.from(allClasses);
}

// === MAIN ===
// Change this path to the crate/module you want to scan:
const TARGET_DIR = path.resolve("./src");

if (!fs.existsSync(TARGET_DIR)) {
  console.error("Target directory not found:", TARGET_DIR);
  process.exit(1);
}

const classes = extractClassesFromDir(TARGET_DIR);

// Output as JSON for Tailwind config
fs.writeFileSync(
  "tailwind_safelist.json",
  JSON.stringify(classes, null, 2)
);

console.log(`Generated safelist with ${classes.length} classes from ${TARGET_DIR}`);
