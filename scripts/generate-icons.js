#!/usr/bin/env node
/**
 * Icon generation script using sharp (Node.js fallback for environments without ImageMagick).
 * Generates all required favicon and app icon assets from the source SVG.
 */

const path = require('path');
const fs = require('fs');

const SOURCE = path.resolve('documentation/static/img/logo.svg');
const OUT_IMG = path.resolve('documentation/static/img');
const ICONS_DIR = path.resolve('documentation/static/img/icons');

const SIZES = [
  { size: 16,  name: 'favicon-16x16.png' },
  { size: 32,  name: 'favicon-32x32.png' },
  { size: 48,  name: 'favicon-48x48.png' },
  { size: 120, name: 'apple-touch-icon-120x120.png' },
  { size: 152, name: 'apple-touch-icon-152x152.png' },
  { size: 167, name: 'apple-touch-icon-167x167.png' },
  { size: 180, name: 'apple-touch-icon.png' },
  { size: 192, name: 'android-chrome-192x192.png' },
  { size: 512, name: 'android-chrome-512x512.png' },
];

async function generatePNGs(sharp) {
  fs.mkdirSync(ICONS_DIR, { recursive: true });
  const svgBuffer = fs.readFileSync(SOURCE);

  for (const { size, name } of SIZES) {
    const outFile = path.join(ICONS_DIR, name);
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(outFile);
    console.log(`  Generated ${size}x${size}: ${outFile}`);
  }
}

/**
 * Build a minimal ICO file from PNG buffers.
 * ICO format: ICONDIR header + ICONDIRENTRY[] + image data
 */
function buildIco(pngBuffers) {
  const count = pngBuffers.length;
  // ICONDIR: 6 bytes
  // ICONDIRENTRY: 16 bytes each
  const headerSize = 6 + 16 * count;
  let dataOffset = headerSize;

  const entries = [];
  for (const buf of pngBuffers) {
    entries.push({ buf, offset: dataOffset });
    dataOffset += buf.length;
  }

  const totalSize = dataOffset;
  const ico = Buffer.alloc(totalSize);

  // ICONDIR header
  ico.writeUInt16LE(0, 0);      // reserved
  ico.writeUInt16LE(1, 2);      // type: 1 = ICO
  ico.writeUInt16LE(count, 4);  // image count

  // ICONDIRENTRY for each image
  for (let i = 0; i < count; i++) {
    const { buf, offset } = entries[i];
    const base = 6 + i * 16;
    // Read dimensions from PNG IHDR (bytes 16-23)
    const w = buf.readUInt32BE(16);
    const h = buf.readUInt32BE(20);
    ico.writeUInt8(w >= 256 ? 0 : w, base);      // width (0 = 256)
    ico.writeUInt8(h >= 256 ? 0 : h, base + 1);  // height (0 = 256)
    ico.writeUInt8(0, base + 2);   // color count (0 = no palette)
    ico.writeUInt8(0, base + 3);   // reserved
    ico.writeUInt16LE(1, base + 4); // color planes
    ico.writeUInt16LE(32, base + 6); // bits per pixel
    ico.writeUInt32LE(buf.length, base + 8);  // image data size
    ico.writeUInt32LE(offset, base + 12);     // image data offset
  }

  // Write image data
  for (const { buf, offset } of entries) {
    buf.copy(ico, offset);
  }

  return ico;
}

async function generateIco(sharp) {
  const icoSizes = [16, 32, 48];
  const svgBuffer = fs.readFileSync(SOURCE);
  const pngBuffers = [];

  for (const size of icoSizes) {
    const buf = await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toBuffer();
    pngBuffers.push(buf);
  }

  const icoBuffer = buildIco(pngBuffers);
  const outFile = path.join(OUT_IMG, 'favicon.ico');
  fs.writeFileSync(outFile, icoBuffer);
  console.log(`  Generated favicon.ico: ${outFile}`);
}

function generateFaviconSvg() {
  const svgContent = fs.readFileSync(SOURCE, 'utf8');
  // Strip comments and editor-specific namespaces
  const cleaned = svgContent
    .replace(/<!--[\s\S]*?-->/g, '')
    .replace(/\s*xmlns:inkscape="[^"]*"/g, '')
    .replace(/\s*xmlns:sodipodi="[^"]*"/g, '')
    .replace(/<metadata[\s\S]*?<\/metadata>/g, '')
    .trim();

  const outFile = path.join(OUT_IMG, 'favicon.svg');
  fs.writeFileSync(outFile, cleaned, 'utf8');
  console.log(`  Generated favicon.svg: ${outFile}`);
}

function checkSizes() {
  const allFiles = [
    path.join(OUT_IMG, 'favicon.ico'),
    path.join(OUT_IMG, 'favicon.svg'),
    ...SIZES.map(({ name }) => path.join(ICONS_DIR, name)),
  ];

  console.log('\nIcon file sizes:');
  let total = 0;
  for (const f of allFiles) {
    if (fs.existsSync(f)) {
      const bytes = fs.statSync(f).size;
      total += bytes;
      console.log(`  ${String(bytes).padStart(8)} bytes  ${f}`);
    } else {
      console.warn(`  MISSING: ${f}`);
    }
  }
  console.log(`  --------`);
  console.log(`  ${String(total).padStart(8)} bytes  total`);

  if (total >= 102400) {
    console.error(`\nWARNING: Total icon asset size (${total} bytes) exceeds 100KB limit (102400 bytes).`);
    process.exit(1);
  }
}

async function main() {
  // Resolve sharp from documentation/node_modules
  const sharpPath = path.resolve('documentation/node_modules/sharp');
  let sharp;
  try {
    sharp = require(sharpPath);
  } catch {
    // Try global/npx sharp
    try {
      sharp = require('sharp');
    } catch {
      console.error('ERROR: sharp module not found. Run: cd documentation && npm install --save-dev sharp');
      process.exit(1);
    }
  }

  if (!fs.existsSync(SOURCE)) {
    console.error(`ERROR: Source SVG not found at '${SOURCE}'`);
    process.exit(1);
  }

  console.log(`Generating icons from ${SOURCE}...`);
  await generatePNGs(sharp);
  await generateIco(sharp);
  generateFaviconSvg();
  checkSizes();
  console.log('\nDone. All icons generated successfully.');
}

main().catch(err => {
  console.error('Error:', err);
  process.exit(1);
});
