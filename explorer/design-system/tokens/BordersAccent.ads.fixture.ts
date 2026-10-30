export const fixture = {
  "title": "BordersAccent",
  "kind": "color",
  "text": "The quick brown fox jumps over the lazy dog",
  "compactText": "Aa",
  "multilineText": "The quick brown fox\njumps over the lazy dog",
  "samples": [
    {"token":"--accent-text","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--border-disabled","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--accent","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--accent-fg","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--accent-hover","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--border-default","property":"background-color","probe":"rgb(31, 97, 163)"},
    {"token":"--border-focus","property":"background-color","probe":"rgb(31, 97, 163)"}
  ]
} as const;
