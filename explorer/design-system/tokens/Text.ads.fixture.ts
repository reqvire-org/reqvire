export const fixture = {
  "title": "Text",
  "kind": "text",
  "text": "The quick brown fox jumps over the lazy dog",
  "compactText": "Aa",
  "multilineText": "The quick brown fox\njumps over the lazy dog",
  "samples": [
    {"token":"--text-disabled","property":"color","probe":"rgb(31, 97, 163)"},
    {"token":"--text-body","property":"color","probe":"rgb(31, 97, 163)"},
    {"token":"--text-inverse","property":"color","probe":"rgb(31, 97, 163)"},
    {"token":"--text-muted","property":"color","probe":"rgb(31, 97, 163)"},
    {"token":"--text-strong","property":"color","probe":"rgb(31, 97, 163)"}
  ]
} as const;
