#!/usr/bin/env bash
# Example from the repo root:
#   cargo run -- model --mmd | scripts/render-mmd.sh -o /tmp/model.svg --scale 1
set -euo pipefail

usage() {
  cat <<'USAGE'
Render Mermaid/MMD text to an image file using Mermaid CLI.

Usage:
  reqvire model --mmd | scripts/render-mmd.sh -o model.svg
  reqvire model --mmd | scripts/render-mmd.sh -o model.png --scale 2
  scripts/render-mmd.sh -i model.mmd -o model.svg

Options:
  -i, --input FILE              Read Mermaid text from FILE instead of stdin
  -o, --output FILE             Output image path (.svg, .png, .pdf, ...)
      --scale N                 Render scale passed to mmdc (default: 2, max for PNG/JPG/WebP: 12)
      --width N                 Browser page width passed to mmdc (default: 800)
      --height N                Browser page height passed to mmdc (default: 600)
  -b, --background COLOR        Background color passed to mmdc (default: #FFFFFF)
      --max-text-size N         Mermaid maxTextSize config value (default: 10000000)
      --max-edges N             Mermaid maxEdges config value (default: 100000)
      --html-labels             Keep Mermaid HTML foreignObject labels instead of SVG-native labels
      --mermaid-config FILE     Mermaid config JSON for mmdc
      --puppeteer-config FILE   Puppeteer config JSON for mmdc
      --mmdc COMMAND            Mermaid CLI command (default: mmdc if available, otherwise npx -y @mermaid-js/mermaid-cli)
  -h, --help                    Show this help
USAGE
}

input_file=""
output_file=""
scale="2"
page_width="800"
page_height="600"
background="#FFFFFF"
max_text_size="10000000"
max_edges="100000"
html_labels="false"
mermaid_config=""
puppeteer_config=""
mmdc_command=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -i|--input)
      input_file="${2:-}"
      shift 2
      ;;
    -o|--output)
      output_file="${2:-}"
      shift 2
      ;;
    --scale)
      scale="${2:-}"
      shift 2
      ;;
    --width)
      page_width="${2:-}"
      shift 2
      ;;
    --height)
      page_height="${2:-}"
      shift 2
      ;;
    -b|--background)
      background="${2:-}"
      shift 2
      ;;
    --max-text-size)
      max_text_size="${2:-}"
      shift 2
      ;;
    --max-edges)
      max_edges="${2:-}"
      shift 2
      ;;
    --html-labels)
      html_labels="true"
      shift
      ;;
    --mermaid-config)
      mermaid_config="${2:-}"
      shift 2
      ;;
    --puppeteer-config)
      puppeteer_config="${2:-}"
      shift 2
      ;;
    --mmdc)
      mmdc_command=("${2:-}")
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument '$1'" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$output_file" ]]; then
  echo "error: --output is required" >&2
  usage >&2
  exit 2
fi

output_lower="${output_file,,}"
case "$scale" in
  ''|*[!0-9.]*|.*|*.*.*)
    echo "error: --scale must be a positive number" >&2
    exit 2
    ;;
esac

if ! awk "BEGIN { exit !($scale > 0) }"; then
  echo "error: --scale must be greater than 0" >&2
  exit 2
fi

case "$page_width" in
  ''|*[!0-9]*)
    echo "error: --width must be a positive integer" >&2
    exit 2
    ;;
esac

case "$page_height" in
  ''|*[!0-9]*)
    echo "error: --height must be a positive integer" >&2
    exit 2
    ;;
esac

if [[ "$page_width" -le 0 || "$page_height" -le 0 ]]; then
  echo "error: --width and --height must be greater than 0" >&2
  exit 2
fi

if [[ "$output_lower" == *.png || "$output_lower" == *.jpg || "$output_lower" == *.jpeg || "$output_lower" == *.webp ]]; then
  if awk "BEGIN { exit !($scale > 12) }"; then
    echo "error: --scale $scale is too large for raster output '$output_file'." >&2
    echo "       Chromium captures PNG/JPG/WebP as bitmaps and can fail on very large diagrams." >&2
    echo "       Use --scale 1..12, or render SVG for zoomable output." >&2
    exit 2
  fi

  if awk "BEGIN { exit !(($page_width * $page_height * $scale * $scale) > 120000000) }"; then
    echo "error: requested raster output is too large: ${page_width}x${page_height} at scale ${scale}." >&2
    echo "       Use a smaller --width/--height/--scale combination, or render SVG for zoomable output." >&2
    exit 2
  fi
fi

tmp_dir="$(mktemp -d -t reqvire-mmd-render-XXXXXX)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

if [[ -z "$input_file" ]]; then
  if [[ -t 0 ]]; then
    echo "error: no --input provided and stdin is empty" >&2
    usage >&2
    exit 2
  fi
  input_file="$tmp_dir/input.mmd"
  cat > "$input_file"
fi

if [[ ! -f "$input_file" ]]; then
  echo "error: input file '$input_file' does not exist" >&2
  exit 2
fi

if [[ -z "$puppeteer_config" ]]; then
  puppeteer_config="$tmp_dir/puppeteer-config.json"
  printf '{"args":["--no-sandbox"]}\n' > "$puppeteer_config"
fi

if [[ -z "$mermaid_config" ]]; then
  mermaid_config="$tmp_dir/mermaid-config.json"
  cat > "$mermaid_config" <<EOF
{
  "theme": "neutral",
  "maxTextSize": $max_text_size,
  "maxEdges": $max_edges,
  "securityLevel": "loose",
  "logLevel": "error",
  "flowchart": {
    "useMaxWidth": true,
    "htmlLabels": $html_labels,
    "curve": "basis",
    "defaultRenderer": "elk"
  },
  "layout": "elk",
  "elk": {
    "mergeEdges": true,
    "nodePlacementStrategy": "SIMPLE"
  },
  "themeVariables": {
    "background": "$background",
    "mainBkg": "$background"
  }
}
EOF
fi

if [[ ${#mmdc_command[@]} -eq 0 ]]; then
  if command -v mmdc >/dev/null 2>&1; then
    mmdc_command=(mmdc)
  else
    mmdc_command=(npx -y @mermaid-js/mermaid-cli)
  fi
fi

"${mmdc_command[@]}" \
  -i "$input_file" \
  -o "$output_file" \
  --width "$page_width" \
  --height "$page_height" \
  --scale "$scale" \
  -b "$background" \
  -c "$mermaid_config" \
  -p "$puppeteer_config"

if [[ "$output_lower" == *.svg ]]; then
  BACKGROUND="$background" perl -0pi -e '
    BEGIN {
      $bg = $ENV{"BACKGROUND"};
      $bg =~ s/&/&amp;/g;
      $bg =~ s/"/&quot;/g;
    }
    s/(<svg\b[^>]*>)/$1<rect data-reqvire-background="true" width="100%" height="100%" fill="$bg"\/>/s
      unless /data-reqvire-background="true"/;
  ' "$output_file"

  if [[ "$html_labels" == "false" ]]; then
    node - "$output_file" <<'NODE'
const fs = require('fs');

const file = process.argv[2];
let svg = fs.readFileSync(file, 'utf8');

function decodeHtml(value) {
  return value
    .replace(/&nbsp;/g, ' ')
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'");
}

function escapeXml(value) {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function labelText(html) {
  return decodeHtml(
    html
      .replace(/<br\s*\/?>/gi, '\n')
      .replace(/<\/p>\s*<p[^>]*>/gi, '\n')
      .replace(/<[^>]+>/g, '')
      .replace(/[ \t\r\f\v]+/g, ' ')
      .replace(/\s*\n\s*/g, '\n')
      .trim()
  );
}

function wrapText(text, width) {
  const maxChars = Math.max(8, Math.floor((Number(width) || 200) / 7.4));
  const lines = [];

  function splitLongToken(token) {
    if (token.length <= maxChars) {
      return [token];
    }

    const parts = [];
    let remaining = token;

    while (remaining.length > maxChars) {
      const window = remaining.slice(0, maxChars + 1);
      let splitAt = Math.max(
        window.lastIndexOf('/'),
        window.lastIndexOf('-'),
        window.lastIndexOf('_'),
        window.lastIndexOf('.')
      );

      if (splitAt < Math.floor(maxChars * 0.45)) {
        splitAt = maxChars;
      } else {
        splitAt += 1;
      }

      parts.push(remaining.slice(0, splitAt));
      remaining = remaining.slice(splitAt);
    }

    if (remaining) {
      parts.push(remaining);
    }

    return parts;
  }

  for (const sourceLine of text.split('\n')) {
    let line = '';
    const words = sourceLine
      .split(/\s+/)
      .filter(Boolean)
      .flatMap(splitLongToken);

    for (const word of words) {
      if (!line) {
        line = word;
        continue;
      }

      if ((line.length + 1 + word.length) <= maxChars) {
        line += ` ${word}`;
      } else {
        lines.push(line);
        line = word;
      }
    }
    if (line) {
      lines.push(line);
    }
  }

  return lines.length > 0 ? lines : [text];
}

svg = svg.replace(
  /(<g class="label"[^>]*>)([\s\S]*?<foreignObject\b([^>]*)>([\s\S]*?)<\/foreignObject>)([\s\S]*?<\/g>)/g,
  (match, prefix, foreignObject, foreignAttrs, innerHtml, suffix) => {
    if (match.includes('data-reqvire-native-label="true"')) {
      return match;
    }

    const width = Number((foreignAttrs.match(/\bwidth="([^"]+)"/) || [])[1]) || 200;
    const height = Number((foreignAttrs.match(/\bheight="([^"]+)"/) || [])[1]) || 24;
    const text = labelText(innerHtml);
    if (!text) {
      return match;
    }

    const lines = wrapText(text, width);
    const fontSize = 16;
    const lineHeight = 18;
    const startY = (height / 2) - (((lines.length - 1) * lineHeight) / 2);
    const tspans = lines
      .map((line, index) => {
        const y = Number((startY + (index * lineHeight)).toFixed(2));
        return `<tspan x="${Number((width / 2).toFixed(2))}" y="${y}">${escapeXml(line)}</tspan>`;
      })
      .join('');

    const hiddenForeignObject = foreignObject.replace(
      /<foreignObject\b/,
      '<foreignObject style="display:none"'
    );
    const nativeText = `<text data-reqvire-native-label="true" text-anchor="middle" dominant-baseline="middle" font-family="trebuchet ms, verdana, arial, sans-serif" font-size="${fontSize}" fill="#000000">${tspans}</text>`;

    return `${prefix}${hiddenForeignObject}${nativeText}${suffix}`;
  }
);

fs.writeFileSync(file, svg);
NODE
  fi
fi

echo "Rendered Mermaid diagram to $output_file"
