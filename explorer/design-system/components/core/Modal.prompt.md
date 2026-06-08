Accessible modal shell with DS structure (`ex-modal-scrim`, `ex-modal`, `ex-modal__head/body/foot`). Use for element detail, ontology node detail, source preview, and confirmation dialogs.

```jsx
<Modal open={open} onOpenChange={setOpen}>
  <ModalContent aria-labelledby="element-title">
    <ModalHeader>
      <TypeBadge type="capability" dot={false} />
      <ModalTitle id="element-title">Defining Model Structure</ModalTitle>
      <ModalClose asChild>
        <IconButton tone="ghost" aria-label="Close"><Icon name="x" /></IconButton>
      </ModalClose>
    </ModalHeader>
    <ModalBody>...</ModalBody>
    <ModalFooter>
      <Button tone="link">Open source page</Button>
      <span className="ex-spacer" />
      <ModalClose asChild><Button tone="primary">Close</Button></ModalClose>
    </ModalFooter>
  </ModalContent>
</Modal>
```

Always provide an accessible title via `aria-labelledby` or visible `ModalTitle`. Use `ModalClose asChild` with `IconButton` or `Button` so close controls keep the DS button contract. Avoid nested modals.
