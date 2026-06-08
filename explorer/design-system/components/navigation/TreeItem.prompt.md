One row of the model-browser tree. The consumer owns the open/selected state and renders rows in order with increasing `depth`; `TreeItem` draws the twist, icon, label and count.

```jsx
<TreeItem label="requirements" icon={<Icon name="folder" size={15} />} count={6} depth={0} expandable open onToggle={…} onSelect={…} />
<TreeItem label="Capabilities.md" icon={<Icon name="file-text" size={15} />} count={3} depth={1} kind="file" selected />
<TreeItem label="Defining Model Structure" icon={<ElementIcon type="capability" size="sm" />} depth={2} kind="element" onSelect={open} />
```
