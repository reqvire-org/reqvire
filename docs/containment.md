# Containment

Interactive tree view showing the physical organization of the model—how elements are structured within folders and files. Click on folders and files to expand/collapse, or click on elements to navigate to their definitions. Use the Expand All/Collapse All buttons to control the tree view.

```d3-tree
{
  "name": "Reqvire root",
  "type": "folder",
  "children": [
    {
      "name": "requirements",
      "type": "folder",
      "children": [
        {
          "name": "Interfaces",
          "type": "folder",
          "children": [
            {
              "name": "Verifications",
              "type": "folder",
              "children": [
                {
                  "name": "CLIVerifications.md",
                  "type": "file",
                  "link": "requirements/Interfaces/Verifications/CLIVerifications.html",
                  "children": [
                    {
                      "name": "CLI Git Commit Hash Flag Test",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/CLIVerifications.html#cli-git-commit-hash-flag-test"
                    },
                    {
                      "name": "CLI Help Structure Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/CLIVerifications.html#cli-help-structure-verification"
                    },
                    {
                      "name": "Verification Traces Element Navigation Test",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/CLIVerifications.html#verification-traces-element-navigation-test"
                    }
                  ]
                },
                {
                  "name": "WebInterfaceVerifications.md",
                  "type": "file",
                  "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html",
                  "children": [
                    {
                      "name": "Attachment Export Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#attachment-export-verification"
                    },
                    {
                      "name": "Containment Attachment Links Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#containment-attachment-links-verification"
                    },
                    {
                      "name": "Diagram Attachment Display Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#diagram-attachment-display-verification"
                    },
                    {
                      "name": "HTML Export Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#html-export-verification"
                    },
                    {
                      "name": "Model View Element Navigation Test",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#model-view-element-navigation-test"
                    },
                    {
                      "name": "Serve Command Verification",
                      "type": "verification",
                      "link": "requirements/Interfaces/Verifications/WebInterfaceVerifications.html#serve-command-verification"
                    }
                  ]
                }
              ]
            },
            {
              "name": "CLI.md",
              "type": "file",
              "link": "requirements/Interfaces/CLI.html",
              "children": [
                {
                  "name": "Attachment Commands",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#attachment-commands"
                },
                {
                  "name": "CLI Add Element Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-add-element-command"
                },
                {
                  "name": "CLI Change Impact Report Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-change-impact-report-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#text-output-formatting"
                    }
                  ]
                },
                {
                  "name": "CLI Containment Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-containment-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Short Mode Behavior",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#short-mode-behavior"
                    }
                  ]
                },
                {
                  "name": "CLI Coverage Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-coverage-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#text-output-formatting"
                    }
                  ]
                },
                {
                  "name": "CLI Generate Diagrams Flag",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-generate-diagrams-flag"
                },
                {
                  "name": "CLI Interface Structure",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-interface-structure"
                },
                {
                  "name": "CLI Lint Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-lint-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#text-output-formatting"
                    }
                  ]
                },
                {
                  "name": "CLI Model Diagram Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-model-diagram-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    }
                  ]
                },
                {
                  "name": "CLI Move Element Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-move-element-command"
                },
                {
                  "name": "CLI Move File Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-move-file-command"
                },
                {
                  "name": "CLI Remove Diagrams Flag",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-remove-diagrams-flag"
                },
                {
                  "name": "CLI Remove Element Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-remove-element-command"
                },
                {
                  "name": "CLI Rename Element Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-rename-element-command"
                },
                {
                  "name": "CLI Search Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-search-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Short Mode Behavior",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#short-mode-behavior"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#text-output-formatting"
                    }
                  ]
                },
                {
                  "name": "CLI Traces Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#cli-traces-command",
                  "children": [
                    {
                      "name": "JSON Output Structure",
                      "type": "attachment-element",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                    },
                    {
                      "name": "Verification Trace Tree Construction",
                      "type": "attachment-element",
                      "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                    }
                  ]
                },
                {
                  "name": "Detailed Error Handling and Logging",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#detailed-error-handling-and-logging"
                },
                {
                  "name": "Format Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#format-command"
                },
                {
                  "name": "Subdirectory Processing Option",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#subdirectory-processing-option",
                  "children": [
                    {
                      "name": "Subdirectory Auto-Detection Behavior",
                      "type": "attachment-element",
                      "link": "requirements/System/Core/Behaviors.html#subdirectory-auto-detection-behavior"
                    }
                  ]
                },
                {
                  "name": "Validate Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#validate-command"
                },
                {
                  "name": "Verification Traces Element Navigation",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/CLI.html#verification-traces-element-navigation"
                }
              ]
            },
            {
              "name": "Interfaces.md",
              "type": "file",
              "link": "requirements/Interfaces/Interfaces.html",
              "children": [
                {
                  "name": "CLI interface",
                  "type": "user-requirement",
                  "link": "requirements/Interfaces/Interfaces.html#cli-interface"
                },
                {
                  "name": "Web Interface",
                  "type": "user-requirement",
                  "link": "requirements/Interfaces/Interfaces.html#web-interface"
                }
              ]
            },
            {
              "name": "WebInterface.md",
              "type": "file",
              "link": "requirements/Interfaces/WebInterface.html",
              "children": [
                {
                  "name": "Attachment Export",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#attachment-export"
                },
                {
                  "name": "Containment View Attachment Links",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#containment-view-attachment-links"
                },
                {
                  "name": "Diagram Attachment Display",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#diagram-attachment-display"
                },
                {
                  "name": "HTML Export",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#html-export"
                },
                {
                  "name": "Model-Centric View Generation",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#model-centric-view-generation"
                },
                {
                  "name": "Model View Element Navigation",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#model-view-element-navigation"
                },
                {
                  "name": "Serve Command",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#serve-command"
                },
                {
                  "name": "Web Interface Color Scheme",
                  "type": "system-requirement",
                  "link": "requirements/Interfaces/WebInterface.html#web-interface-color-scheme"
                }
              ]
            }
          ]
        },
        {
          "name": "System",
          "type": "folder",
          "children": [
            {
              "name": "Core",
              "type": "folder",
              "children": [
                {
                  "name": "DesignDocuments",
                  "type": "folder",
                  "children": [
                    {
                      "name": "ElementIdentity.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/ElementIdentity.html"
                    },
                    {
                      "name": "IdentifiersAndRelations.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/IdentifiersAndRelations.html"
                    },
                    {
                      "name": "MarkdownStructure.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/MarkdownStructure.html"
                    },
                    {
                      "name": "RefinementElements.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/RefinementElements.html"
                    },
                    {
                      "name": "RelationTypes.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/RelationTypes.html"
                    },
                    {
                      "name": "ReservedSubsections.md",
                      "type": "design-document",
                      "link": "requirements/System/Core/DesignDocuments/ReservedSubsections.html"
                    }
                  ]
                },
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "AttachmentsVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html",
                      "children": [
                        {
                          "name": "Attach Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attach-command-verification"
                        },
                        {
                          "name": "Attachment Identifier CRUD Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachment-identifier-crud-verification"
                        },
                        {
                          "name": "Attachment Output Rendering Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachment-output-rendering-verification"
                        },
                        {
                          "name": "Attachment Search Filters Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachment-search-filters-verification"
                        },
                        {
                          "name": "Attachments Change Impact Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachments-change-impact-verification"
                        },
                        {
                          "name": "Attachments Subsection Parsing Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachments-subsection-parsing-verification"
                        },
                        {
                          "name": "Attachments Validation Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#attachments-validation-verification"
                        },
                        {
                          "name": "Detach Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#detach-command-verification"
                        },
                        {
                          "name": "Move Attachment Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#move-attachment-command-verification"
                        },
                        {
                          "name": "Remove Attachment Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/AttachmentsVerifications.html#remove-attachment-command-verification"
                        }
                      ]
                    },
                    {
                      "name": "ParsingVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Core/Verifications/ParsingVerifications.html",
                      "children": [
                        {
                          "name": "Element Subsection Parsing Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#element-subsection-parsing-test"
                        },
                        {
                          "name": "Fragment Normalization Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#fragment-normalization-test"
                        },
                        {
                          "name": "Non-Reserved Subsections Content Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#non-reserved-subsections-content-test"
                        },
                        {
                          "name": "Refinement Element Type Parsing Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#refinement-element-type-parsing-test"
                        },
                        {
                          "name": "Refinement Relations Rejection Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#refinement-relations-rejection-test"
                        },
                        {
                          "name": "Specification File Identification Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ParsingVerifications.html#specification-file-identification-test"
                        }
                      ]
                    },
                    {
                      "name": "ValidationVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Core/Verifications/ValidationVerifications.html",
                      "children": [
                        {
                          "name": "Default Element Type Assignment Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#default-element-type-assignment-test"
                        },
                        {
                          "name": "Element Type Relation Compatibility Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#element-type-relation-compatibility-test"
                        },
                        {
                          "name": "File Exclusion Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#file-exclusion-test"
                        },
                        {
                          "name": "Invalid Header Structure Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#invalid-header-structure-test"
                        },
                        {
                          "name": "Invalid Relations Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#invalid-relations-test"
                        },
                        {
                          "name": "Requirements Files Search and Detection Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#requirements-files-search-and-detection-test"
                        },
                        {
                          "name": "Same-File Fragment Relations Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#same-file-fragment-relations-test"
                        },
                        {
                          "name": "Subdirectory Processing Verification",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#subdirectory-processing-verification"
                        },
                        {
                          "name": "Unstructured Documents Test",
                          "type": "verification",
                          "link": "requirements/System/Core/Verifications/ValidationVerifications.html#unstructured-documents-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/System/Core/Behaviors.html",
                  "children": [
                    {
                      "name": "Attachment Identifier CRUD Update Behavior",
                      "type": "refinement",
                      "link": "requirements/System/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                    },
                    {
                      "name": "Attachment Input Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/System/Core/Behaviors.html#attachment-input-auto-detection-behavior"
                    },
                    {
                      "name": "Subdirectory Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/System/Core/Behaviors.html#subdirectory-auto-detection-behavior"
                    }
                  ]
                },
                {
                  "name": "Configuration.md",
                  "type": "file",
                  "link": "requirements/System/Core/Configuration.html",
                  "children": [
                    {
                      "name": "Ignore Files Integration",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Configuration.html#ignore-files-integration"
                    },
                    {
                      "name": "Ignoring Unstructured Documents",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Configuration.html#ignoring-unstructured-documents"
                    },
                    {
                      "name": "Requirements Processing",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Configuration.html#requirements-processing"
                    },
                    {
                      "name": "Reserved Repository Files Exclusion",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Configuration.html#reserved-repository-files-exclusion"
                    },
                    {
                      "name": "Structured Markdown Files Search and Detection",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Configuration.html#structured-markdown-files-search-and-detection"
                    }
                  ]
                },
                {
                  "name": "ModelManagement.md",
                  "type": "file",
                  "link": "requirements/System/Core/ModelManagement.html",
                  "children": [
                    {
                      "name": "Attachment Identifier Updates",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#attachment-identifier-updates",
                      "children": [
                        {
                          "name": "Attachment Identifier CRUD Update Behavior",
                          "type": "attachment-element",
                          "link": "requirements/System/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Coexistence of Structured and Unstructured Documents",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#coexistence-of-structured-and-unstructured-documents"
                    },
                    {
                      "name": "Default Requirement Type Assignment",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#default-requirement-type-assignment"
                    },
                    {
                      "name": "Efficient Processing",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#efficient-processing"
                    },
                    {
                      "name": "Element Manipulation Operations",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#element-manipulation-operations"
                    },
                    {
                      "name": "Element Type Relation Compatibility",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#element-type-relation-compatibility",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/RelationTypes.md"
                        }
                      ]
                    },
                    {
                      "name": "Git Repository as Project Root",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#git-repository-as-project-root"
                    },
                    {
                      "name": "Refinement Element Structure Constraints",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#refinement-element-structure-constraints"
                    },
                    {
                      "name": "Relation Types and behaviors",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#relation-types-and-behaviors",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/RelationTypes.md"
                        }
                      ]
                    },
                    {
                      "name": "Supported Element Types",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#supported-element-types"
                    },
                    {
                      "name": "Template-Based Model Bootstrapping",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#template-based-model-bootstrapping"
                    },
                    {
                      "name": "Verification Type Categories",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/ModelManagement.html#verification-type-categories"
                    }
                  ]
                },
                {
                  "name": "StructureAndParsing.md",
                  "type": "file",
                  "link": "requirements/System/Core/StructureAndParsing.html",
                  "children": [
                    {
                      "name": "Attachment Target Validation",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#attachment-target-validation"
                    },
                    {
                      "name": "Element Identity Model",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#element-identity-model",
                      "children": [
                        {
                          "name": "ElementIdentity.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/ElementIdentity.md"
                        }
                      ]
                    },
                    {
                      "name": "Identifiers and Relations",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#identifiers-and-relations",
                      "children": [
                        {
                          "name": "IdentifiersAndRelations.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/IdentifiersAndRelations.md"
                        }
                      ]
                    },
                    {
                      "name": "Reserved Subsections Support",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#reserved-subsections-support",
                      "children": [
                        {
                          "name": "ReservedSubsections.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/ReservedSubsections.md"
                        }
                      ]
                    },
                    {
                      "name": "Specification File Identification",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#specification-file-identification"
                    },
                    {
                      "name": "Structure and Addressing in Markdown Documents",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/StructureAndParsing.html#structure-and-addressing-in-markdown-documents",
                      "children": [
                        {
                          "name": "MarkdownStructure.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Core/DesignDocuments/MarkdownStructure.md"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Validation.md",
                  "type": "file",
                  "link": "requirements/System/Core/Validation.html",
                  "children": [
                    {
                      "name": "Cross-Component Dependency Validator",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#cross-component-dependency-validator"
                    },
                    {
                      "name": "Enhanced Validation Error Reporting",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#enhanced-validation-error-reporting"
                    },
                    {
                      "name": "Excluded File Relation Validation",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#excluded-file-relation-validation"
                    },
                    {
                      "name": "GraphRegistry as Primary Registry",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#graphregistry-as-primary-registry"
                    },
                    {
                      "name": "Integrated Validation",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#integrated-validation"
                    },
                    {
                      "name": "Internal Consistency Validator",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#internal-consistency-validator"
                    },
                    {
                      "name": "Markdown Structure Validator",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#markdown-structure-validator"
                    },
                    {
                      "name": "Relation Element Type Validator",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#relation-element-type-validator"
                    },
                    {
                      "name": "Relation Type Validation",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#relation-type-validation"
                    },
                    {
                      "name": "Two-Pass Validation Strategy",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#two-pass-validation-strategy"
                    },
                    {
                      "name": "Validate Cross-Component Dependencies",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#validate-cross-component-dependencies"
                    },
                    {
                      "name": "Validate Filesystem Structure",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#validate-filesystem-structure"
                    },
                    {
                      "name": "Validate Internal Consistency",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#validate-internal-consistency"
                    },
                    {
                      "name": "Validate Markdown Structure",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#validate-markdown-structure"
                    },
                    {
                      "name": "Validate Relation Types",
                      "type": "user-requirement",
                      "link": "requirements/System/Core/Validation.html#validate-relation-types"
                    },
                    {
                      "name": "Validation Error Handling",
                      "type": "system-requirement",
                      "link": "requirements/System/Core/Validation.html#validation-error-handling"
                    }
                  ]
                }
              ]
            },
            {
              "name": "Integration",
              "type": "folder",
              "children": [
                {
                  "name": "CodeAlignment.md",
                  "type": "file",
                  "link": "requirements/System/Integration/CodeAlignment.html",
                  "children": [
                    {
                      "name": "BAT style comment",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#bat-style-comment"
                    },
                    {
                      "name": "Code Traceability",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#code-traceability"
                    },
                    {
                      "name": "Comment Style by File Extension",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#comment-style-by-file-extension"
                    },
                    {
                      "name": "CSS style comment",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#css-style-comment"
                    },
                    {
                      "name": "Dash style comment",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#dash-style-comment"
                    },
                    {
                      "name": "Slash style comment",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#slash-style-comment"
                    },
                    {
                      "name": "SQL style comment",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#sql-style-comment"
                    },
                    {
                      "name": "Suggest Code Refactoring",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#suggest-code-refactoring"
                    },
                    {
                      "name": "Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#traceability-format"
                    },
                    {
                      "name": "Validating Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#validating-traceability-format"
                    },
                    {
                      "name": "XML style comment",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/CodeAlignment.html#xml-style-comment"
                    }
                  ]
                },
                {
                  "name": "GitHubIntegration.md",
                  "type": "file",
                  "link": "requirements/System/Integration/GitHubIntegration.html",
                  "children": [
                    {
                      "name": "Automate Documentation Export",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/GitHubIntegration.html#automate-documentation-export"
                    },
                    {
                      "name": "Automate Pull Request Validations",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/GitHubIntegration.html#automate-pull-request-validations"
                    },
                    {
                      "name": "Automated Documentation Export on PR Merge",
                      "type": "system-requirement",
                      "link": "requirements/System/Integration/GitHubIntegration.html#automated-documentation-export-on-pr-merge"
                    },
                    {
                      "name": "Generate Change Logs for Pull Requests",
                      "type": "user-requirement",
                      "link": "requirements/System/Integration/GitHubIntegration.html#generate-change-logs-for-pull-requests"
                    }
                  ]
                }
              ]
            },
            {
              "name": "Operations",
              "type": "folder",
              "children": [
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "ElementManipulationVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html",
                      "children": [
                        {
                          "name": "CLI Add Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#cli-add-element-test"
                        },
                        {
                          "name": "CLI Move Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#cli-move-element-test"
                        },
                        {
                          "name": "CLI Move File Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#cli-move-file-test"
                        },
                        {
                          "name": "CLI Remove Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#cli-remove-element-test"
                        },
                        {
                          "name": "CLI Rename Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#cli-rename-element-test"
                        },
                        {
                          "name": "Create Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#create-element-test"
                        },
                        {
                          "name": "Delete Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#delete-element-test"
                        },
                        {
                          "name": "File Persistence Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#file-persistence-test"
                        },
                        {
                          "name": "Move Element Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#move-element-test"
                        },
                        {
                          "name": "Move File Squash Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#move-file-squash-test"
                        },
                        {
                          "name": "Relation Consistency Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#relation-consistency-test"
                        },
                        {
                          "name": "Target Location Validation Test",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/ElementManipulationVerifications.html#target-location-validation-test"
                        }
                      ]
                    },
                    {
                      "name": "FormattingVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Operations/Verifications/FormattingVerifications.html",
                      "children": [
                        {
                          "name": "Format Command Requirements Verification",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/FormattingVerifications.html#format-command-requirements-verification"
                        }
                      ]
                    },
                    {
                      "name": "LintingVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Operations/Verifications/LintingVerifications.html",
                      "children": [
                        {
                          "name": "Lint Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Operations/Verifications/LintingVerifications.html#lint-command-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "ElementManipulation.md",
                  "type": "file",
                  "link": "requirements/System/Operations/ElementManipulation.html",
                  "children": [
                    {
                      "name": "Create Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#create-element-operation"
                    },
                    {
                      "name": "Delete Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#delete-element-operation"
                    },
                    {
                      "name": "Element Manipulation File Persistence",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#element-manipulation-file-persistence"
                    },
                    {
                      "name": "Move Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#move-element-operation"
                    },
                    {
                      "name": "Move File Operation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#move-file-operation"
                    },
                    {
                      "name": "Relation Consistency Maintenance",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#relation-consistency-maintenance"
                    },
                    {
                      "name": "Rename Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#rename-element-operation"
                    },
                    {
                      "name": "Target Location Validation and Auto-Creation",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/ElementManipulation.html#target-location-validation-and-auto-creation"
                    }
                  ]
                },
                {
                  "name": "Formatting.md",
                  "type": "file",
                  "link": "requirements/System/Operations/Formatting.html",
                  "children": [
                    {
                      "name": "Document Structure Normalization",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Formatting.html#document-structure-normalization"
                    },
                    {
                      "name": "File Pattern Exclusion for Format",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Formatting.html#file-pattern-exclusion-for-format"
                    },
                    {
                      "name": "Format Consistency Enforcement",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Formatting.html#format-consistency-enforcement"
                    },
                    {
                      "name": "Formatting Output",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Formatting.html#formatting-output"
                    },
                    {
                      "name": "Git-Style Diff Output for Format",
                      "type": "user-requirement",
                      "link": "requirements/System/Operations/Formatting.html#git-style-diff-output-for-format"
                    },
                    {
                      "name": "Model Formatting",
                      "type": "user-requirement",
                      "link": "requirements/System/Operations/Formatting.html#model-formatting"
                    },
                    {
                      "name": "Replace Absolute Links with Relative Links",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Formatting.html#replace-absolute-links-with-relative-links"
                    }
                  ]
                },
                {
                  "name": "Linting.md",
                  "type": "file",
                  "link": "requirements/System/Operations/Linting.html",
                  "children": [
                    {
                      "name": "Lint Auto-fix Capability",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#lint-auto-fix-capability"
                    },
                    {
                      "name": "Lint Output Formatting",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#lint-output-formatting"
                    },
                    {
                      "name": "Model Linting",
                      "type": "user-requirement",
                      "link": "requirements/System/Operations/Linting.html#model-linting"
                    },
                    {
                      "name": "Multi-Branch Convergence Detection",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#multi-branch-convergence-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Hierarchical Relations Detection",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#redundant-hierarchical-relations-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Verify Relations Detection",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#redundant-verify-relations-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Safe Redundant Hierarchical Relations Auto-Removal",
                      "type": "system-requirement",
                      "link": "requirements/System/Operations/Linting.html#safe-redundant-hierarchical-relations-auto-removal"
                    }
                  ]
                }
              ]
            },
            {
              "name": "Output",
              "type": "folder",
              "children": [
                {
                  "name": "DesignDocuments",
                  "type": "folder",
                  "children": [
                    {
                      "name": "ContainmentView.md",
                      "type": "design-document",
                      "link": "requirements/System/Output/DesignDocuments/ContainmentView.html"
                    },
                    {
                      "name": "OutputFormats.md",
                      "type": "design-document",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html"
                    },
                    {
                      "name": "SearchFiltering.md",
                      "type": "design-document",
                      "link": "requirements/System/Output/DesignDocuments/SearchFiltering.html"
                    },
                    {
                      "name": "OutputFormats.md",
                      "type": "file",
                      "link": "requirements/System/Output/DesignDocuments/OutputFormats.html",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "refinement",
                          "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#json-output-structure"
                        },
                        {
                          "name": "Short Mode Behavior",
                          "type": "refinement",
                          "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#short-mode-behavior"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "refinement",
                          "link": "requirements/System/Output/DesignDocuments/OutputFormats.html#text-output-formatting"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "DiagramVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Output/Verifications/DiagramVerifications.html",
                      "children": [
                        {
                          "name": "Automated Documentation Export on PR Merge Verification",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#automated-documentation-export-on-pr-merge-verification"
                        },
                        {
                          "name": "Diagram Generation Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#diagram-generation-test"
                        },
                        {
                          "name": "Diagram Relation Filtering Verification",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#diagram-relation-filtering-verification"
                        },
                        {
                          "name": "File Diagram Attachment Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#file-diagram-attachment-test"
                        },
                        {
                          "name": "Remove Generated Diagrams Verification",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#remove-generated-diagrams-verification"
                        },
                        {
                          "name": "Visualize Model Relationships Verification",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/DiagramVerifications.html#visualize-model-relationships-verification"
                        }
                      ]
                    },
                    {
                      "name": "ReportingVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Output/Verifications/ReportingVerifications.html",
                      "children": [
                        {
                          "name": "Containment Hierarchy Extraction Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#containment-hierarchy-extraction-test"
                        },
                        {
                          "name": "Containment View Design Documents Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#containment-view-design-documents-test"
                        },
                        {
                          "name": "Containment View JSON Output Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#containment-view-json-output-test"
                        },
                        {
                          "name": "Containment View Mermaid Diagram Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#containment-view-mermaid-diagram-test"
                        },
                        {
                          "name": "Containment View Text Output Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#containment-view-text-output-test"
                        },
                        {
                          "name": "Custom Element Type Tracking Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#custom-element-type-tracking-test"
                        },
                        {
                          "name": "HTML Export Containment View Integration Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#html-export-containment-view-integration-test"
                        },
                        {
                          "name": "Model Command Verification",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#model-command-verification"
                        },
                        {
                          "name": "Search Command Tests",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#search-command-tests"
                        },
                        {
                          "name": "Verification Coverage Report Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#verification-coverage-report-test"
                        },
                        {
                          "name": "Verification Traces Filter Options Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#verification-traces-filter-options-test"
                        },
                        {
                          "name": "Verification Traces From-Folder Test",
                          "type": "verification",
                          "link": "requirements/System/Output/Verifications/ReportingVerifications.html#verification-traces-from-folder-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "DiagramGeneration.md",
                  "type": "file",
                  "link": "requirements/System/Output/DiagramGeneration.html",
                  "children": [
                    {
                      "name": "Complete Model Structure Visualization",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#complete-model-structure-visualization"
                    },
                    {
                      "name": "Diagram Generation",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#diagram-generation"
                    },
                    {
                      "name": "Diagram Removal",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#diagram-removal"
                    },
                    {
                      "name": "File Diagram Attachment Display",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#file-diagram-attachment-display"
                    },
                    {
                      "name": "Interactive Mermaid Diagram Node Behavior",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#interactive-mermaid-diagram-node-behavior"
                    },
                    {
                      "name": "Interactive Mermaid Diagrams",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#interactive-mermaid-diagrams"
                    },
                    {
                      "name": "Model Visualization and Exploration",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#model-visualization-and-exploration"
                    },
                    {
                      "name": "Remove Generated Diagrams",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#remove-generated-diagrams"
                    },
                    {
                      "name": "SysML-Compatible Relationship Rendering",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#sysml-compatible-relationship-rendering"
                    },
                    {
                      "name": "Trace Relation Non-Directional Behavior",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/DiagramGeneration.html#trace-relation-non-directional-behavior"
                    }
                  ]
                },
                {
                  "name": "Reporting.md",
                  "type": "file",
                  "link": "requirements/System/Output/Reporting.html",
                  "children": [
                    {
                      "name": "Containment View Design Documents",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#containment-view-design-documents"
                    },
                    {
                      "name": "Containment View Report",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#containment-view-report"
                    },
                    {
                      "name": "Containment View Report Generation",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#containment-view-report-generation",
                      "children": [
                        {
                          "name": "ContainmentView.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Output/DesignDocuments/ContainmentView.md"
                        }
                      ]
                    },
                    {
                      "name": "Custom Element Type Tracking",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#custom-element-type-tracking"
                    },
                    {
                      "name": "Forward-Only Relation Traversal",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#forward-only-relation-traversal"
                    },
                    {
                      "name": "Model Diagram Output Formats",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#model-diagram-output-formats"
                    },
                    {
                      "name": "Model Reports",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#model-reports"
                    },
                    {
                      "name": "Model Structure and Summaries",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#model-structure-and-summaries"
                    },
                    {
                      "name": "Provide Validation Reports",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#provide-validation-reports"
                    },
                    {
                      "name": "Search Fine Grained Filtering",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#search-fine-grained-filtering",
                      "children": [
                        {
                          "name": "SearchFiltering.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Output/DesignDocuments/SearchFiltering.md"
                        }
                      ]
                    },
                    {
                      "name": "Search Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#search-report-generator"
                    },
                    {
                      "name": "Tracing Structural Changes",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#tracing-structural-changes"
                    },
                    {
                      "name": "Validation Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#validation-report-generator"
                    },
                    {
                      "name": "Verification Coverage Report",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#verification-coverage-report"
                    },
                    {
                      "name": "Verification Coverage Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/System/Output/Reporting.html#verification-coverage-report-generator"
                    },
                    {
                      "name": "Verification Upward Traceability",
                      "type": "user-requirement",
                      "link": "requirements/System/Output/Reporting.html#verification-upward-traceability"
                    }
                  ]
                }
              ]
            },
            {
              "name": "Processing",
              "type": "folder",
              "children": [
                {
                  "name": "DesignDocuments",
                  "type": "folder",
                  "children": [
                    {
                      "name": "ChangePropagation.md",
                      "type": "design-document",
                      "link": "requirements/System/Processing/DesignDocuments/ChangePropagation.html"
                    },
                    {
                      "name": "VerificationTraceAlgorithm.md",
                      "type": "design-document",
                      "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html"
                    },
                    {
                      "name": "VerificationTraceAlgorithm.md",
                      "type": "file",
                      "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "refinement",
                          "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "ChangeImpactVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html",
                      "children": [
                        {
                          "name": "Change Impact Analysis Verification",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#change-impact-analysis-verification"
                        },
                        {
                          "name": "Change Impact Detection Test",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#change-impact-detection-test"
                        },
                        {
                          "name": "Change Impact Relations Test",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#change-impact-relations-test"
                        },
                        {
                          "name": "Change Impact Smart Filtering Test",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#change-impact-smart-filtering-test"
                        },
                        {
                          "name": "Element Content Extraction Test",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#element-content-extraction-test"
                        },
                        {
                          "name": "Structural Change Reports Verification",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/ChangeImpactVerifications.html#structural-change-reports-verification"
                        }
                      ]
                    },
                    {
                      "name": "TraceVerifications.md",
                      "type": "file",
                      "link": "requirements/System/Processing/Verifications/TraceVerifications.html",
                      "children": [
                        {
                          "name": "Trace Relations No Cycles Verification",
                          "type": "verification",
                          "link": "requirements/System/Processing/Verifications/TraceVerifications.html#trace-relations-no-cycles-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "ChangeImpact.md",
                  "type": "file",
                  "link": "requirements/System/Processing/ChangeImpact.html",
                  "children": [
                    {
                      "name": "Change Impact Detection",
                      "type": "system-requirement",
                      "link": "requirements/System/Processing/ChangeImpact.html#change-impact-detection"
                    },
                    {
                      "name": "Requirements Change Propagation",
                      "type": "system-requirement",
                      "link": "requirements/System/Processing/ChangeImpact.html#requirements-change-propagation",
                      "children": [
                        {
                          "name": "ChangePropagation.md",
                          "type": "attachment-file",
                          "link": "requirements/System/Processing/DesignDocuments/ChangePropagation.md"
                        }
                      ]
                    },
                    {
                      "name": "Structural Change Analyzer",
                      "type": "system-requirement",
                      "link": "requirements/System/Processing/ChangeImpact.html#structural-change-analyzer"
                    }
                  ]
                },
                {
                  "name": "VerificationTraces.md",
                  "type": "file",
                  "link": "requirements/System/Processing/VerificationTraces.html",
                  "children": [
                    {
                      "name": "Verification Roll-up Strategy",
                      "type": "system-requirement",
                      "link": "requirements/System/Processing/VerificationTraces.html#verification-roll-up-strategy"
                    },
                    {
                      "name": "Verification Trace Builder",
                      "type": "system-requirement",
                      "link": "requirements/System/Processing/VerificationTraces.html#verification-trace-builder",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/System/Processing/DesignDocuments/VerificationTraceAlgorithm.html#verification-trace-tree-construction"
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "name": "UserStories.md",
          "type": "file",
          "link": "requirements/UserStories.html",
          "children": [
            {
              "name": "AI-Assisted System Model Management",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#ai-assisted-system-model-management"
            },
            {
              "name": "Align with Industry Standards",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#align-with-industry-standards"
            },
            {
              "name": "Aligning Design with Code",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#aligning-design-with-code"
            },
            {
              "name": "Fostering Community Contributions",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#fostering-community-contributions"
            },
            {
              "name": "Generate Diagrams",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#generate-diagrams"
            },
            {
              "name": "Integrate with GitHub Workflows",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#integrate-with-github-workflows"
            },
            {
              "name": "Managing System Models",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#managing-system-models"
            },
            {
              "name": "Model Export",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#model-export"
            },
            {
              "name": "Promote Automation and Efficiency",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#promote-automation-and-efficiency"
            },
            {
              "name": "Provide Reports",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#provide-reports"
            },
            {
              "name": "Trace Changes in System Model",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#trace-changes-in-system-model"
            },
            {
              "name": "Validating Structures",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#validating-structures"
            }
          ]
        }
      ]
    }
  ]
}
```
