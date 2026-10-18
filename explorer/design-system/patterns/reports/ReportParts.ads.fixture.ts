export const fixture = {
  "title": "ReportParts",
  "scopes": [
    {
      "identifier": "whole",
      "name": "Whole model",
      "depth": 0
    }
  ],
  "segments": [
    {
      "value": 75,
      "colorToken": "--success"
    },
    {
      "value": 25,
      "colorToken": "--warning"
    }
  ],
  "rows": [
    {
      "label": "Directly verified",
      "value": "9 requirements"
    },
    {
      "label": "Total in tree",
      "value": "12 requirements"
    }
  ]
} as const;
