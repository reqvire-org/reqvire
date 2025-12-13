# Containment

The containment view shows the physical organization of the model—how requirements, verifications, and other elements are structured within folders and files. This hierarchical view helps you understand the model's file structure and navigate to specific elements.

<div class="view-toggle">
    <button id="btn-sunburst" class="view-btn active" onclick="showView('sunburst')">Sunburst</button>
    <button id="btn-icicle" class="view-btn" onclick="showView('icicle')">Icicle</button>
</div>

<div id="view-sunburst" class="containment-view">

<p class="view-instructions">Click on segments to zoom in. Click center circle to zoom out. Click the center name link to navigate to the element.</p>

```d3-sunburst
{
  "name": "Reqvire root",
  "type": "folder",
  "children": [
    {
      "name": "requirements",
      "type": "folder",
      "children": [
        {
          "name": "Functional",
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
                      "link": "requirements/Functional/Core/DesignDocuments/ElementIdentity.html"
                    },
                    {
                      "name": "IdentifiersAndRelations.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/IdentifiersAndRelations.html"
                    },
                    {
                      "name": "MarkdownStructure.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/MarkdownStructure.html"
                    },
                    {
                      "name": "RelationTypes.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.html"
                    },
                    {
                      "name": "ReservedSubsections.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.html"
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
                      "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html",
                      "children": [
                        {
                          "name": "Attach Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attach-command-verification"
                        },
                        {
                          "name": "Attachment Identifier CRUD Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-identifier-crud-verification"
                        },
                        {
                          "name": "Attachment Output Rendering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-output-rendering-verification"
                        },
                        {
                          "name": "Attachment Search Filters Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-search-filters-verification"
                        },
                        {
                          "name": "Attachments Change Impact Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-change-impact-verification"
                        },
                        {
                          "name": "Attachments Subsection Parsing Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-subsection-parsing-verification"
                        },
                        {
                          "name": "Attachments Validation Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-validation-verification"
                        },
                        {
                          "name": "Detach Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#detach-command-verification"
                        },
                        {
                          "name": "Move Asset Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#move-asset-command-verification"
                        },
                        {
                          "name": "Remove Asset Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#remove-asset-command-verification"
                        }
                      ]
                    },
                    {
                      "name": "ParsingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html",
                      "children": [
                        {
                          "name": "Element Subsection Parsing Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#element-subsection-parsing-test"
                        },
                        {
                          "name": "Fragment Normalization Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#fragment-normalization-test"
                        },
                        {
                          "name": "Non-Reserved Subsections Content Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#non-reserved-subsections-content-test"
                        },
                        {
                          "name": "Refinement Element Type Parsing Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#refinement-element-type-parsing-test"
                        },
                        {
                          "name": "Refinement Relations Rejection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#refinement-relations-rejection-test"
                        },
                        {
                          "name": "Specification File Identification Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#specification-file-identification-test"
                        }
                      ]
                    },
                    {
                      "name": "ValidationVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html",
                      "children": [
                        {
                          "name": "Cross-Section Duplicate Validation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#cross-section-duplicate-validation-test"
                        },
                        {
                          "name": "Default Element Type Assignment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#default-element-type-assignment-test"
                        },
                        {
                          "name": "Element Type Relation Compatibility Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#element-type-relation-compatibility-test"
                        },
                        {
                          "name": "File Exclusion Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#file-exclusion-test"
                        },
                        {
                          "name": "Invalid Header Structure Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#invalid-header-structure-test"
                        },
                        {
                          "name": "Invalid Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#invalid-relations-test"
                        },
                        {
                          "name": "Requirements Files Search and Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#requirements-files-search-and-detection-test"
                        },
                        {
                          "name": "Same-File Fragment Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#same-file-fragment-relations-test"
                        },
                        {
                          "name": "Subdirectory Processing Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#subdirectory-processing-verification"
                        },
                        {
                          "name": "Type Validation Errors Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#type-validation-errors-test",
                          "children": [
                            {
                              "name": "Type Validation Error Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Unstructured Documents Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#unstructured-documents-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Behaviors.html",
                  "children": [
                    {
                      "name": "Attachment Identifier CRUD Update Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                    },
                    {
                      "name": "Attachment Input Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#attachment-input-auto-detection-behavior"
                    },
                    {
                      "name": "Subdirectory Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#subdirectory-auto-detection-behavior"
                    },
                    {
                      "name": "Two-Pass Validation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                    },
                    {
                      "name": "Type Validation Error Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                    },
                    {
                      "name": "Validation Error Reporting Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                    }
                  ]
                },
                {
                  "name": "Configuration.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Configuration.html",
                  "children": [
                    {
                      "name": "Coexistence of Structured and Unstructured Documents",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#coexistence-of-structured-and-unstructured-documents",
                      "children": [
                        {
                          "name": "Refinement Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#refinement-specification"
                        }
                      ]
                    },
                    {
                      "name": "Ignore Files Integration",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#ignore-files-integration"
                    },
                    {
                      "name": "Ignoring Unstructured Documents",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#ignoring-unstructured-documents"
                    },
                    {
                      "name": "Requirements Processing",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#requirements-processing"
                    },
                    {
                      "name": "Reserved Repository Files Exclusion",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#reserved-repository-files-exclusion"
                    },
                    {
                      "name": "Structured Markdown Files Search and Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#structured-markdown-files-search-and-detection"
                    }
                  ]
                },
                {
                  "name": "Constraints.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Constraints.html",
                  "children": [
                    {
                      "name": "Cross-Section Duplicate Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Constraints.html#cross-section-duplicate-constraint"
                    },
                    {
                      "name": "Element Type Relation Compatibility Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Constraints.html#element-type-relation-compatibility-constraint"
                    }
                  ]
                },
                {
                  "name": "ModelManagement.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/ModelManagement.html",
                  "children": [
                    {
                      "name": "Attachment Identifier Updates",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#attachment-identifier-updates",
                      "children": [
                        {
                          "name": "Attachment Identifier CRUD Update Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Default Requirement Type Assignment",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#default-requirement-type-assignment",
                      "children": [
                        {
                          "name": "Element Type Metadata Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#element-type-metadata-specification"
                        }
                      ]
                    },
                    {
                      "name": "Efficient Processing",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#efficient-processing"
                    },
                    {
                      "name": "Element Manipulation Operations",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#element-manipulation-operations",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Element Type Relation Compatibility",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#element-type-relation-compatibility",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    },
                    {
                      "name": "Git Repository as Project Root",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#git-repository-as-project-root",
                      "children": [
                        {
                          "name": "Containment Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#containment-specification"
                        }
                      ]
                    },
                    {
                      "name": "Refinement Element Structure Constraints",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#refinement-element-structure-constraints",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    },
                    {
                      "name": "Relation Management Operations",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#relation-management-operations"
                    },
                    {
                      "name": "Relation Types and behaviors",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#relation-types-and-behaviors",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Relation Semantics Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#relation-semantics-specification"
                        }
                      ]
                    },
                    {
                      "name": "Template-Based Model Bootstrapping",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#template-based-model-bootstrapping"
                    },
                    {
                      "name": "Verification Type Categories",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#verification-type-categories",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Specifications.html",
                  "children": [
                    {
                      "name": "Element Type Metadata Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#element-type-metadata-specification"
                    },
                    {
                      "name": "Git Repository Scope Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                    },
                    {
                      "name": "Ignore Files Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#ignore-files-specification"
                    },
                    {
                      "name": "Requirements Processing Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#requirements-processing-specification"
                    },
                    {
                      "name": "Reserved Files Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#reserved-files-specification"
                    },
                    {
                      "name": "Verification Type Selection Guidelines",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                    }
                  ]
                },
                {
                  "name": "StructureAndParsing.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/StructureAndParsing.html",
                  "children": [
                    {
                      "name": "Element Identity Model",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#element-identity-model",
                      "children": [
                        {
                          "name": "ElementIdentity.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ElementIdentity.md"
                        }
                      ]
                    },
                    {
                      "name": "Identifiers and Relations",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#identifiers-and-relations",
                      "children": [
                        {
                          "name": "IdentifiersAndRelations.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/IdentifiersAndRelations.md"
                        }
                      ]
                    },
                    {
                      "name": "Reserved Subsections Support",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#reserved-subsections-support",
                      "children": [
                        {
                          "name": "ReservedSubsections.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.md"
                        }
                      ]
                    },
                    {
                      "name": "Specification File Identification",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#specification-file-identification"
                    },
                    {
                      "name": "Structure and Addressing in Markdown Documents",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#structure-and-addressing-in-markdown-documents",
                      "children": [
                        {
                          "name": "MarkdownStructure.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/MarkdownStructure.md"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Validation.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Validation.html",
                  "children": [
                    {
                      "name": "Attachment Target Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#attachment-target-validation",
                      "children": [
                        {
                          "name": "ReservedSubsections.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.md"
                        }
                      ]
                    },
                    {
                      "name": "Cross-Component Dependency Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#cross-component-dependency-validator"
                    },
                    {
                      "name": "Cross-Section Duplicate Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#cross-section-duplicate-validation"
                    },
                    {
                      "name": "Enhanced Validation Error Reporting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#enhanced-validation-error-reporting"
                    },
                    {
                      "name": "Excluded File Relation Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#excluded-file-relation-validation"
                    },
                    {
                      "name": "GraphRegistry as Primary Registry",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#graphregistry-as-primary-registry",
                      "children": [
                        {
                          "name": "Requirements Processing Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#requirements-processing-specification"
                        }
                      ]
                    },
                    {
                      "name": "Integrated Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#integrated-validation",
                      "children": [
                        {
                          "name": "Two-Pass Validation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Internal Consistency Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#internal-consistency-validator"
                    },
                    {
                      "name": "Markdown Structure Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#markdown-structure-validator"
                    },
                    {
                      "name": "Relation Element Type Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#relation-element-type-validator"
                    },
                    {
                      "name": "Relation Type Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#relation-type-validation"
                    },
                    {
                      "name": "Two-Pass Validation Strategy",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#two-pass-validation-strategy"
                    },
                    {
                      "name": "Type Validation Error Requirement",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#type-validation-error-requirement"
                    },
                    {
                      "name": "Validate Cross-Component Dependencies",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-cross-component-dependencies"
                    },
                    {
                      "name": "Validate Filesystem Structure",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-filesystem-structure"
                    },
                    {
                      "name": "Validate Internal Consistency",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-internal-consistency"
                    },
                    {
                      "name": "Validate Markdown Structure",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-markdown-structure"
                    },
                    {
                      "name": "Validate Relation Types",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-relation-types",
                      "children": [
                        {
                          "name": "Relation Semantics Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#relation-semantics-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validation Error Handling",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validation-error-handling",
                      "children": [
                        {
                          "name": "Error Message Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                        }
                      ]
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
                  "link": "requirements/Functional/Integration/CodeAlignment.html",
                  "children": [
                    {
                      "name": "BAT style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#bat-style-comment"
                    },
                    {
                      "name": "Code Traceability",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#code-traceability"
                    },
                    {
                      "name": "Comment Style by File Extension",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#comment-style-by-file-extension"
                    },
                    {
                      "name": "CSS style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#css-style-comment"
                    },
                    {
                      "name": "Dash style comment",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#dash-style-comment"
                    },
                    {
                      "name": "Slash style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#slash-style-comment"
                    },
                    {
                      "name": "SQL style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#sql-style-comment"
                    },
                    {
                      "name": "Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#traceability-format"
                    },
                    {
                      "name": "Validating Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#validating-traceability-format"
                    },
                    {
                      "name": "XML style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#xml-style-comment"
                    }
                  ]
                },
                {
                  "name": "GitHubIntegration.md",
                  "type": "file",
                  "link": "requirements/Functional/Integration/GitHubIntegration.html",
                  "children": [
                    {
                      "name": "Automate Documentation Export",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automate-documentation-export"
                    },
                    {
                      "name": "Automate Pull Request Validations",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automate-pull-request-validations"
                    },
                    {
                      "name": "Automated Documentation Export on PR Merge",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automated-documentation-export-on-pr-merge"
                    },
                    {
                      "name": "Generate Change Logs for Pull Requests",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#generate-change-logs-for-pull-requests"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Integration/Specifications.html",
                  "children": [
                    {
                      "name": "Comment Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Integration/Specifications.html#comment-style-specification"
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
                      "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html",
                      "children": [
                        {
                          "name": "Add Command Duplicate Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#add-command-duplicate-detection-test"
                        },
                        {
                          "name": "Add Command Error Messages Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#add-command-error-messages-test"
                        },
                        {
                          "name": "CLI Add Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-add-element-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "CLI Move Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-move-element-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "CLI Move File Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-move-file-test"
                        },
                        {
                          "name": "CLI Remove Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-remove-element-test"
                        },
                        {
                          "name": "CLI Rename Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-rename-element-test"
                        },
                        {
                          "name": "Create Element Override Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#create-element-override-test",
                          "children": [
                            {
                              "name": "Create Element Override Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Create Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#create-element-test"
                        },
                        {
                          "name": "Delete Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#delete-element-test"
                        },
                        {
                          "name": "File Persistence Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#file-persistence-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Link Command Cross-Section Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#link-command-cross-section-detection-test"
                        },
                        {
                          "name": "Link Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#link-command-verification"
                        },
                        {
                          "name": "Merge Elements Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#merge-elements-test"
                        },
                        {
                          "name": "Move Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#move-element-test"
                        },
                        {
                          "name": "Move File Squash Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#move-file-squash-test"
                        },
                        {
                          "name": "Relation Consistency Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#relation-consistency-test"
                        },
                        {
                          "name": "Target Location Validation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#target-location-validation-test"
                        },
                        {
                          "name": "Unlink Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#unlink-command-verification"
                        }
                      ]
                    },
                    {
                      "name": "FormattingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html",
                      "children": [
                        {
                          "name": "Element Ordering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#element-ordering-verification",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Format Command Requirements Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#format-command-requirements-verification"
                        },
                        {
                          "name": "Format Duplicate Removal Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#format-duplicate-removal-test"
                        },
                        {
                          "name": "Full Relations Insertion Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#full-relations-insertion-verification"
                        },
                        {
                          "name": "Relation Ordering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#relation-ordering-verification"
                        }
                      ]
                    },
                    {
                      "name": "LintingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html",
                      "children": [
                        {
                          "name": "Lint Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html#lint-command-verification"
                        },
                        {
                          "name": "Redundant Hierarchical Attachment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html#redundant-hierarchical-attachment-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Behaviors.html",
                  "children": [
                    {
                      "name": "Create Element Override Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                    },
                    {
                      "name": "Dry-Run Mode Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                    },
                    {
                      "name": "Element Ordering Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                    },
                    {
                      "name": "File Persistence Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                    },
                    {
                      "name": "Format Duplicate Removal Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#format-duplicate-removal-behavior"
                    },
                    {
                      "name": "Merge Content Transformation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#merge-content-transformation-behavior"
                    }
                  ]
                },
                {
                  "name": "Constraints.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Constraints.html",
                  "children": [
                    {
                      "name": "Merge Type Compatibility Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Constraints.html#merge-type-compatibility-constraint"
                    },
                    {
                      "name": "Target Location Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                    }
                  ]
                },
                {
                  "name": "ElementManipulation.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/ElementManipulation.html",
                  "children": [
                    {
                      "name": "Create Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#create-element-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Delete Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#delete-element-operation"
                    },
                    {
                      "name": "Element Manipulation File Persistence",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#element-manipulation-file-persistence",
                      "children": [
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Merge Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#merge-element-operation"
                    },
                    {
                      "name": "Move Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#move-element-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Move File Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#move-file-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        }
                      ]
                    },
                    {
                      "name": "Relation Consistency Maintenance",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#relation-consistency-maintenance"
                    },
                    {
                      "name": "Rename Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#rename-element-operation"
                    },
                    {
                      "name": "Target Location Validation and Auto-Creation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#target-location-validation-and-auto-creation",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Formatting.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Formatting.html",
                  "children": [
                    {
                      "name": "Document Structure Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#document-structure-normalization"
                    },
                    {
                      "name": "Element Ordering Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#element-ordering-normalization",
                      "children": [
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "File Pattern Exclusion for Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#file-pattern-exclusion-for-format"
                    },
                    {
                      "name": "Format Consistency Enforcement",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#format-consistency-enforcement"
                    },
                    {
                      "name": "Format Duplicate Removal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#format-duplicate-removal"
                    },
                    {
                      "name": "Formatting Output",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#formatting-output"
                    },
                    {
                      "name": "Full Relations Insertion",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#full-relations-insertion",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        }
                      ]
                    },
                    {
                      "name": "Git-Style Diff Output for Format",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#git-style-diff-output-for-format"
                    },
                    {
                      "name": "Model Formatting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#model-formatting"
                    },
                    {
                      "name": "Relation Ordering Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#relation-ordering-normalization"
                    },
                    {
                      "name": "Replace Absolute Links with Relative Links",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#replace-absolute-links-with-relative-links"
                    }
                  ]
                },
                {
                  "name": "Linting.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Linting.html",
                  "children": [
                    {
                      "name": "Lint Auto-fix Capability",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#lint-auto-fix-capability",
                      "children": [
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Linting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#model-linting"
                    },
                    {
                      "name": "Multi-Branch Convergence Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#multi-branch-convergence-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Hierarchical Attachment Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-hierarchical-attachment-detection"
                    },
                    {
                      "name": "Redundant Hierarchical Relations Detection and Auto-Removal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-hierarchical-relations-detection-and-auto-removal",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Verify Relations Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-verify-relations-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Specifications.html",
                  "children": [
                    {
                      "name": "Create Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#create-element-workflow-specification"
                    },
                    {
                      "name": "Delete Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#delete-element-workflow-specification"
                    },
                    {
                      "name": "Document Structure Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#document-structure-specification"
                    },
                    {
                      "name": "Format Consistency Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#format-consistency-specification"
                    },
                    {
                      "name": "Lint Output Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#lint-output-specification"
                    },
                    {
                      "name": "Merge Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#merge-element-workflow-specification"
                    },
                    {
                      "name": "Move Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#move-element-workflow-specification"
                    },
                    {
                      "name": "Multi-Branch Convergence Detection Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#multi-branch-convergence-detection-specification"
                    },
                    {
                      "name": "Orphaned Children Error Message Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#orphaned-children-error-message-specification"
                    },
                    {
                      "name": "Redundant Hierarchical Relations Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#redundant-hierarchical-relations-specification"
                    },
                    {
                      "name": "Relation Operations Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-operations-specification"
                    },
                    {
                      "name": "Relation Ordering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-ordering-specification"
                    },
                    {
                      "name": "Relation Validation Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-validation-specification"
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
                      "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.html"
                    },
                    {
                      "name": "SearchFiltering.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Output/DesignDocuments/SearchFiltering.html"
                    },
                    {
                      "name": "TraceFlowView.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Output/DesignDocuments/TraceFlowView.html"
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
                      "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html",
                      "children": [
                        {
                          "name": "Automated Documentation Export on PR Merge Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#automated-documentation-export-on-pr-merge-verification"
                        },
                        {
                          "name": "Diagram Generation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#diagram-generation-test"
                        },
                        {
                          "name": "Diagram Relation Filtering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#diagram-relation-filtering-verification"
                        },
                        {
                          "name": "File Diagram Attachment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#file-diagram-attachment-test"
                        },
                        {
                          "name": "Visualize Model Relationships Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#visualize-model-relationships-verification"
                        }
                      ]
                    },
                    {
                      "name": "ReportingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html",
                      "children": [
                        {
                          "name": "CLI Collect Command Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#cli-collect-command-test"
                        },
                        {
                          "name": "Containment Hierarchy Extraction Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-hierarchy-extraction-test"
                        },
                        {
                          "name": "Containment View Design Documents Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-design-documents-test"
                        },
                        {
                          "name": "Containment View JSON Output Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-json-output-test"
                        },
                        {
                          "name": "Containment View Mermaid Diagram Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-mermaid-diagram-test"
                        },
                        {
                          "name": "Containment View Text Output Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-text-output-test"
                        },
                        {
                          "name": "Custom Element Type Tracking Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#custom-element-type-tracking-test"
                        },
                        {
                          "name": "HTML Export Containment View Integration Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#html-export-containment-view-integration-test"
                        },
                        {
                          "name": "Model Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#model-command-verification"
                        },
                        {
                          "name": "Multi-Type Search Filter Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#multi-type-search-filter-test"
                        },
                        {
                          "name": "Resources Report Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#resources-report-verification"
                        },
                        {
                          "name": "Reverse Model Traversal Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#reverse-model-traversal-test",
                          "children": [
                            {
                              "name": "Reverse Relation Traversal Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Search Command Tests",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#search-command-tests"
                        },
                        {
                          "name": "Start Type Filter Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#start-type-filter-test",
                          "children": [
                            {
                              "name": "Start Element Type Filter Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                            }
                          ]
                        },
                        {
                          "name": "TraceFlow View Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#traceflow-view-test"
                        },
                        {
                          "name": "Verification Coverage Report Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-coverage-report-test"
                        },
                        {
                          "name": "Verification Traces Filter Options Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-traces-filter-options-test"
                        },
                        {
                          "name": "Verification Traces From-Folder Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-traces-from-folder-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Behaviors.html",
                  "children": [
                    {
                      "name": "Mermaid Diagram Interaction Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#mermaid-diagram-interaction-behavior"
                    },
                    {
                      "name": "Reverse Relation Traversal Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                    },
                    {
                      "name": "Short Mode Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                    },
                    {
                      "name": "Start Element Type Filter Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                    },
                    {
                      "name": "Verification Coverage Philosophy Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#verification-coverage-philosophy-behavior"
                    }
                  ]
                },
                {
                  "name": "DiagramGeneration.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/DiagramGeneration.html",
                  "children": [
                    {
                      "name": "Diagram Generation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#diagram-generation",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "File Diagram Attachment Display",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#file-diagram-attachment-display",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "Interactive Mermaid Diagram Node Behavior",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#interactive-mermaid-diagram-node-behavior",
                      "children": [
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "Interactive Mermaid Diagrams",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#interactive-mermaid-diagrams",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "SysML-Compatible Relationship Rendering",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#sysml-compatible-relationship-rendering",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Trace Relation Non-Directional Behavior",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#trace-relation-non-directional-behavior"
                    }
                  ]
                },
                {
                  "name": "Reporting.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Reporting.html",
                  "children": [
                    {
                      "name": "Collect Content from Requirement Chain",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#collect-content-from-requirement-chain",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Comma-Separated Type Filter Parsing",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#comma-separated-type-filter-parsing"
                    },
                    {
                      "name": "Containment View Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#containment-view-report",
                      "children": [
                        {
                          "name": "ContainmentView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.md"
                        },
                        {
                          "name": "Containment Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#containment-specification"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Resources Report Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Flexible Search Type Filtering",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#flexible-search-type-filtering"
                    },
                    {
                      "name": "Forward-Only Relation Traversal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#forward-only-relation-traversal",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Diagram Output Formats",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-diagram-output-formats",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Reports",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-reports",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Structure and Summaries",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-structure-and-summaries"
                    },
                    {
                      "name": "Provide Validation Reports",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#provide-validation-reports"
                    },
                    {
                      "name": "Resources Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#resources-report"
                    },
                    {
                      "name": "Reverse Relation Traversal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#reverse-relation-traversal",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Search Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#search-report-generator",
                      "children": [
                        {
                          "name": "SearchFiltering.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/SearchFiltering.md"
                        },
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Resources Report Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Start Element Type Filtering",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#start-element-type-filtering"
                    },
                    {
                      "name": "TraceFlow View Report Generation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#traceflow-view-report-generation",
                      "children": [
                        {
                          "name": "TraceFlowView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/TraceFlowView.md"
                        },
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        },
                        {
                          "name": "Verification Roll-up Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                        },
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Tracing Structural Changes",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#tracing-structural-changes",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validation Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#validation-report-generator",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Verification Coverage Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#verification-coverage-report",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Verification Coverage Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#verification-coverage-specification"
                        },
                        {
                          "name": "Verification Roll-up Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                        },
                        {
                          "name": "Verification Type Selection Guidelines",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Specifications.html",
                  "children": [
                    {
                      "name": "Collect Content Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#collect-content-specification"
                    },
                    {
                      "name": "Collect Output Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#collect-output-format-specification"
                    },
                    {
                      "name": "Color Scheme Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#color-scheme-specification"
                    },
                    {
                      "name": "Deterministic Output Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                    },
                    {
                      "name": "Diagram Relation Filtering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                    },
                    {
                      "name": "Diff Output Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                    },
                    {
                      "name": "Error Message Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                    },
                    {
                      "name": "JSON Output Structure",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                    },
                    {
                      "name": "Markdown Report Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#markdown-report-style-specification"
                    },
                    {
                      "name": "Mermaid Diagram Generation Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                    },
                    {
                      "name": "Mermaid Diagram Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                    },
                    {
                      "name": "Mermaid Interactive Features Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                    },
                    {
                      "name": "Resources Report Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                    },
                    {
                      "name": "SysML Rendering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#sysml-rendering-specification"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                    },
                    {
                      "name": "Verification Trace Diagram Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#verification-trace-diagram-specification"
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
                      "link": "requirements/Functional/Processing/DesignDocuments/ChangePropagation.html"
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
                      "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html",
                      "children": [
                        {
                          "name": "Change Impact Analysis Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-analysis-verification"
                        },
                        {
                          "name": "Change Impact Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-detection-test"
                        },
                        {
                          "name": "Change Impact Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-relations-test"
                        },
                        {
                          "name": "Change Impact Smart Filtering Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-smart-filtering-test"
                        },
                        {
                          "name": "Element Content Extraction Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#element-content-extraction-test"
                        },
                        {
                          "name": "Structural Change Reports Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#structural-change-reports-verification"
                        }
                      ]
                    },
                    {
                      "name": "TraceVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Processing/Verifications/TraceVerifications.html",
                      "children": [
                        {
                          "name": "Trace Relations No Cycles Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/TraceVerifications.html#trace-relations-no-cycles-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/Behaviors.html",
                  "children": [
                    {
                      "name": "Change Propagation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Behaviors.html#change-propagation-behavior"
                    }
                  ]
                },
                {
                  "name": "ChangeImpact.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/ChangeImpact.html",
                  "children": [
                    {
                      "name": "Change Impact Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#change-impact-detection"
                    },
                    {
                      "name": "Requirements Change Propagation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#requirements-change-propagation",
                      "children": [
                        {
                          "name": "ChangePropagation.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Processing/DesignDocuments/ChangePropagation.md"
                        }
                      ]
                    },
                    {
                      "name": "Structural Change Analyzer",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#structural-change-analyzer"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/Specifications.html",
                  "children": [
                    {
                      "name": "Verification Roll-up Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                    },
                    {
                      "name": "Verification Trace Tree Construction",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                    }
                  ]
                },
                {
                  "name": "VerificationTraces.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/VerificationTraces.html",
                  "children": [
                    {
                      "name": "Verification Roll-up Strategy",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-roll-up-strategy"
                    },
                    {
                      "name": "Verification Trace Builder",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-trace-builder"
                    },
                    {
                      "name": "Verification Upward Traceability",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-upward-traceability",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
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
          "name": "Interfaces",
          "type": "folder",
          "children": [
            {
              "name": "CLI",
              "type": "folder",
              "children": [
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "CLIVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html",
                      "children": [
                        {
                          "name": "CLI Git Commit Hash Flag Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#cli-git-commit-hash-flag-test"
                        },
                        {
                          "name": "CLI Help Structure Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#cli-help-structure-verification"
                        },
                        {
                          "name": "Verification Traces Element Navigation Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#verification-traces-element-navigation-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Commands.md",
                  "type": "file",
                  "link": "requirements/Interfaces/CLI/Commands.html",
                  "children": [
                    {
                      "name": "Attachment Commands",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#attachment-commands",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "Attachment Input Auto-Detection Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#attachment-input-auto-detection-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Add Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-add-element-command",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        },
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Create Element Override Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Create Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#create-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Change Impact Report Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-change-impact-report-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Change Propagation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Behaviors.html#change-propagation-behavior"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Collect Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-collect-command",
                      "children": [
                        {
                          "name": "Collect Content Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#collect-content-specification"
                        },
                        {
                          "name": "Collect Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#collect-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Containment Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-containment-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Short Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "ContainmentView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.md"
                        },
                        {
                          "name": "D3.js Containment Tree Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Coverage Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-coverage-command",
                      "children": [
                        {
                          "name": "Verification Type Selection Guidelines",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        }
                      ]
                    },
                    {
                      "name": "CLI Interface Structure",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-interface-structure"
                    },
                    {
                      "name": "CLI Lint Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-lint-command",
                      "children": [
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Lint Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#lint-output-specification"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Multi-Branch Convergence Detection Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#multi-branch-convergence-detection-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Merge Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-merge-element-command",
                      "children": [
                        {
                          "name": "Merge Content Transformation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#merge-content-transformation-behavior"
                        },
                        {
                          "name": "Merge Type Compatibility Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#merge-type-compatibility-constraint"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Merge Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#merge-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Model Diagram Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-model-diagram-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "Reverse Relation Traversal Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                        },
                        {
                          "name": "Start Element Type Filter Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move Asset Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-asset-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-element-command",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        },
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Move Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#move-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move File Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-file-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "CLI Remove Asset Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-remove-asset-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Remove Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-remove-element-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Delete Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#delete-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Rename Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-rename-element-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "CLI Resources Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-resources-command",
                      "children": [
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        }
                      ]
                    },
                    {
                      "name": "CLI Search Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-search-command",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Short Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Traces Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-traces-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Detailed Error Handling and Logging",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#detailed-error-handling-and-logging",
                      "children": [
                        {
                          "name": "Validation Error Reporting Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Format Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#format-command",
                      "children": [
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "Relation Commands",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#relation-commands",
                      "children": [
                        {
                          "name": "Relation Operations Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#relation-operations-specification"
                        },
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validate Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#validate-command",
                      "children": [
                        {
                          "name": "Two-Pass Validation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                        },
                        {
                          "name": "Validation Error Reporting Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Error Message Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Verification Traces Element Navigation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#verification-traces-element-navigation"
                    }
                  ]
                }
              ]
            },
            {
              "name": "WebInterface",
              "type": "folder",
              "children": [
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "HTMLGenerationVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html",
                      "children": [
                        {
                          "name": "Component Reuse Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#component-reuse-verification"
                        },
                        {
                          "name": "HTML Validity Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#html-validity-verification"
                        },
                        {
                          "name": "Integration Test Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#integration-test-verification"
                        },
                        {
                          "name": "Mobile Responsiveness Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#mobile-responsiveness-verification"
                        },
                        {
                          "name": "Responsive Design Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#responsive-design-verification"
                        }
                      ]
                    },
                    {
                      "name": "WebInterfaceVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html",
                      "children": [
                        {
                          "name": "Attachment Export Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#attachment-export-verification"
                        },
                        {
                          "name": "Containment Attachment Links Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#containment-attachment-links-verification"
                        },
                        {
                          "name": "Diagram Attachment Display Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#diagram-attachment-display-verification"
                        },
                        {
                          "name": "HTML Export Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#html-export-verification"
                        },
                        {
                          "name": "Model View Element Navigation Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#model-view-element-navigation-test"
                        },
                        {
                          "name": "Serve Command Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#serve-command-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Behaviors.html",
                  "children": [
                    {
                      "name": "Web Interface Navigation Behavior",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Behaviors.html#web-interface-navigation-behavior"
                    }
                  ]
                },
                {
                  "name": "Features.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Features.html",
                  "children": [
                    {
                      "name": "Attachment Export",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#attachment-export"
                    },
                    {
                      "name": "Containment View Attachment Links",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#containment-view-attachment-links",
                      "children": [
                        {
                          "name": "D3.js Containment Tree Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                        }
                      ]
                    },
                    {
                      "name": "Diagram Attachment Display",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#diagram-attachment-display",
                      "children": [
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        }
                      ]
                    },
                    {
                      "name": "HTML Export",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#html-export",
                      "children": [
                        {
                          "name": "Web Interface Navigation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Behaviors.html#web-interface-navigation-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Model-Centric View Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#model-centric-view-generation",
                      "children": [
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "HTML Export Pipeline Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                        },
                        {
                          "name": "HTML Navigation Bar Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model View Element Navigation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#model-view-element-navigation"
                    },
                    {
                      "name": "Serve Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#serve-command",
                      "children": [
                        {
                          "name": "HTML Export Pipeline Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                        }
                      ]
                    },
                    {
                      "name": "Web Interface Color Scheme",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#web-interface-color-scheme",
                      "children": [
                        {
                          "name": "HTML Navigation Bar Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                        },
                        {
                          "name": "HTML Branding Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-branding-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "HTMLGeneration.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html",
                  "children": [
                    {
                      "name": "Component-Based HTML Architecture",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#component-based-html-architecture"
                    },
                    {
                      "name": "CSS Framework Integration",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#css-framework-integration"
                    },
                    {
                      "name": "Mobile-Friendly Documentation",
                      "type": "user-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#mobile-friendly-documentation"
                    },
                    {
                      "name": "Responsive HTML Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#responsive-html-generation"
                    },
                    {
                      "name": "Type-Safe HTML Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#type-safe-html-generation"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Specifications.html",
                  "children": [
                    {
                      "name": "D3.js Containment Tree Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                    },
                    {
                      "name": "HTML Branding Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-branding-specification"
                    },
                    {
                      "name": "HTML Export Pipeline Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                    },
                    {
                      "name": "HTML Navigation Bar Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                    },
                    {
                      "name": "Web Interface Style Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#web-interface-style-specification"
                    }
                  ]
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
            }
          ]
        },
        {
          "name": "Refinements.md",
          "type": "file",
          "link": "requirements/Refinements.html",
          "children": [
            {
              "name": "Containment Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#containment-specification"
            },
            {
              "name": "Refinement Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#refinement-specification"
            },
            {
              "name": "Relation Semantics Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#relation-semantics-specification"
            },
            {
              "name": "Supported Element Types Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#supported-element-types-specification"
            },
            {
              "name": "Traceability Reporting Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#traceability-reporting-specification"
            },
            {
              "name": "Verification Coverage Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#verification-coverage-specification"
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
              "name": "Defining Model Structure",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#defining-model-structure"
            },
            {
              "name": "Formatting Model Documents",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#formatting-model-documents"
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
              "name": "Linting Model Quality",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#linting-model-quality"
            },
            {
              "name": "Operating on Model Elements",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#operating-on-model-elements"
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
              "name": "System Model Interfaces",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#system-model-interfaces"
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
            },
            {
              "name": "Verification Traceability",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#verification-traceability"
            }
          ]
        }
      ]
    }
  ]
}
```


</div>

<div id="view-icicle" class="containment-view">

<p class="view-instructions">Click on bars to zoom in. Click breadcrumb path to navigate back. Click the element link to open it.</p>

```d3-icicle
{
  "name": "Reqvire root",
  "type": "folder",
  "children": [
    {
      "name": "requirements",
      "type": "folder",
      "children": [
        {
          "name": "Functional",
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
                      "link": "requirements/Functional/Core/DesignDocuments/ElementIdentity.html"
                    },
                    {
                      "name": "IdentifiersAndRelations.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/IdentifiersAndRelations.html"
                    },
                    {
                      "name": "MarkdownStructure.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/MarkdownStructure.html"
                    },
                    {
                      "name": "RelationTypes.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.html"
                    },
                    {
                      "name": "ReservedSubsections.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.html"
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
                      "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html",
                      "children": [
                        {
                          "name": "Attach Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attach-command-verification"
                        },
                        {
                          "name": "Attachment Identifier CRUD Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-identifier-crud-verification"
                        },
                        {
                          "name": "Attachment Output Rendering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-output-rendering-verification"
                        },
                        {
                          "name": "Attachment Search Filters Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachment-search-filters-verification"
                        },
                        {
                          "name": "Attachments Change Impact Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-change-impact-verification"
                        },
                        {
                          "name": "Attachments Subsection Parsing Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-subsection-parsing-verification"
                        },
                        {
                          "name": "Attachments Validation Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#attachments-validation-verification"
                        },
                        {
                          "name": "Detach Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#detach-command-verification"
                        },
                        {
                          "name": "Move Asset Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#move-asset-command-verification"
                        },
                        {
                          "name": "Remove Asset Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/AttachmentsVerifications.html#remove-asset-command-verification"
                        }
                      ]
                    },
                    {
                      "name": "ParsingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html",
                      "children": [
                        {
                          "name": "Element Subsection Parsing Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#element-subsection-parsing-test"
                        },
                        {
                          "name": "Fragment Normalization Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#fragment-normalization-test"
                        },
                        {
                          "name": "Non-Reserved Subsections Content Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#non-reserved-subsections-content-test"
                        },
                        {
                          "name": "Refinement Element Type Parsing Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#refinement-element-type-parsing-test"
                        },
                        {
                          "name": "Refinement Relations Rejection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#refinement-relations-rejection-test"
                        },
                        {
                          "name": "Specification File Identification Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ParsingVerifications.html#specification-file-identification-test"
                        }
                      ]
                    },
                    {
                      "name": "ValidationVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html",
                      "children": [
                        {
                          "name": "Cross-Section Duplicate Validation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#cross-section-duplicate-validation-test"
                        },
                        {
                          "name": "Default Element Type Assignment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#default-element-type-assignment-test"
                        },
                        {
                          "name": "Element Type Relation Compatibility Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#element-type-relation-compatibility-test"
                        },
                        {
                          "name": "File Exclusion Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#file-exclusion-test"
                        },
                        {
                          "name": "Invalid Header Structure Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#invalid-header-structure-test"
                        },
                        {
                          "name": "Invalid Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#invalid-relations-test"
                        },
                        {
                          "name": "Requirements Files Search and Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#requirements-files-search-and-detection-test"
                        },
                        {
                          "name": "Same-File Fragment Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#same-file-fragment-relations-test"
                        },
                        {
                          "name": "Subdirectory Processing Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#subdirectory-processing-verification"
                        },
                        {
                          "name": "Type Validation Errors Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#type-validation-errors-test",
                          "children": [
                            {
                              "name": "Type Validation Error Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Unstructured Documents Test",
                          "type": "verification",
                          "link": "requirements/Functional/Core/Verifications/ValidationVerifications.html#unstructured-documents-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Behaviors.html",
                  "children": [
                    {
                      "name": "Attachment Identifier CRUD Update Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                    },
                    {
                      "name": "Attachment Input Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#attachment-input-auto-detection-behavior"
                    },
                    {
                      "name": "Subdirectory Auto-Detection Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#subdirectory-auto-detection-behavior"
                    },
                    {
                      "name": "Two-Pass Validation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                    },
                    {
                      "name": "Type Validation Error Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                    },
                    {
                      "name": "Validation Error Reporting Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                    }
                  ]
                },
                {
                  "name": "Configuration.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Configuration.html",
                  "children": [
                    {
                      "name": "Coexistence of Structured and Unstructured Documents",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#coexistence-of-structured-and-unstructured-documents",
                      "children": [
                        {
                          "name": "Refinement Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#refinement-specification"
                        }
                      ]
                    },
                    {
                      "name": "Ignore Files Integration",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#ignore-files-integration"
                    },
                    {
                      "name": "Ignoring Unstructured Documents",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#ignoring-unstructured-documents"
                    },
                    {
                      "name": "Requirements Processing",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#requirements-processing"
                    },
                    {
                      "name": "Reserved Repository Files Exclusion",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#reserved-repository-files-exclusion"
                    },
                    {
                      "name": "Structured Markdown Files Search and Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Configuration.html#structured-markdown-files-search-and-detection"
                    }
                  ]
                },
                {
                  "name": "Constraints.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Constraints.html",
                  "children": [
                    {
                      "name": "Cross-Section Duplicate Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Constraints.html#cross-section-duplicate-constraint"
                    },
                    {
                      "name": "Element Type Relation Compatibility Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Constraints.html#element-type-relation-compatibility-constraint"
                    }
                  ]
                },
                {
                  "name": "ModelManagement.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/ModelManagement.html",
                  "children": [
                    {
                      "name": "Attachment Identifier Updates",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#attachment-identifier-updates",
                      "children": [
                        {
                          "name": "Attachment Identifier CRUD Update Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#attachment-identifier-crud-update-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Default Requirement Type Assignment",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#default-requirement-type-assignment",
                      "children": [
                        {
                          "name": "Element Type Metadata Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#element-type-metadata-specification"
                        }
                      ]
                    },
                    {
                      "name": "Efficient Processing",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#efficient-processing"
                    },
                    {
                      "name": "Element Manipulation Operations",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#element-manipulation-operations",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Element Type Relation Compatibility",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#element-type-relation-compatibility",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    },
                    {
                      "name": "Git Repository as Project Root",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#git-repository-as-project-root",
                      "children": [
                        {
                          "name": "Containment Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#containment-specification"
                        }
                      ]
                    },
                    {
                      "name": "Refinement Element Structure Constraints",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#refinement-element-structure-constraints",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    },
                    {
                      "name": "Relation Management Operations",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#relation-management-operations"
                    },
                    {
                      "name": "Relation Types and behaviors",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#relation-types-and-behaviors",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Relation Semantics Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#relation-semantics-specification"
                        }
                      ]
                    },
                    {
                      "name": "Template-Based Model Bootstrapping",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#template-based-model-bootstrapping"
                    },
                    {
                      "name": "Verification Type Categories",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/ModelManagement.html#verification-type-categories",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Specifications.html",
                  "children": [
                    {
                      "name": "Element Type Metadata Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#element-type-metadata-specification"
                    },
                    {
                      "name": "Git Repository Scope Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                    },
                    {
                      "name": "Ignore Files Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#ignore-files-specification"
                    },
                    {
                      "name": "Requirements Processing Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#requirements-processing-specification"
                    },
                    {
                      "name": "Reserved Files Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#reserved-files-specification"
                    },
                    {
                      "name": "Verification Type Selection Guidelines",
                      "type": "refinement",
                      "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                    }
                  ]
                },
                {
                  "name": "StructureAndParsing.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/StructureAndParsing.html",
                  "children": [
                    {
                      "name": "Element Identity Model",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#element-identity-model",
                      "children": [
                        {
                          "name": "ElementIdentity.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ElementIdentity.md"
                        }
                      ]
                    },
                    {
                      "name": "Identifiers and Relations",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#identifiers-and-relations",
                      "children": [
                        {
                          "name": "IdentifiersAndRelations.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/IdentifiersAndRelations.md"
                        }
                      ]
                    },
                    {
                      "name": "Reserved Subsections Support",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#reserved-subsections-support",
                      "children": [
                        {
                          "name": "ReservedSubsections.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.md"
                        }
                      ]
                    },
                    {
                      "name": "Specification File Identification",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#specification-file-identification"
                    },
                    {
                      "name": "Structure and Addressing in Markdown Documents",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/StructureAndParsing.html#structure-and-addressing-in-markdown-documents",
                      "children": [
                        {
                          "name": "MarkdownStructure.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/MarkdownStructure.md"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Validation.md",
                  "type": "file",
                  "link": "requirements/Functional/Core/Validation.html",
                  "children": [
                    {
                      "name": "Attachment Target Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#attachment-target-validation",
                      "children": [
                        {
                          "name": "ReservedSubsections.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/ReservedSubsections.md"
                        }
                      ]
                    },
                    {
                      "name": "Cross-Component Dependency Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#cross-component-dependency-validator"
                    },
                    {
                      "name": "Cross-Section Duplicate Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#cross-section-duplicate-validation"
                    },
                    {
                      "name": "Enhanced Validation Error Reporting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#enhanced-validation-error-reporting"
                    },
                    {
                      "name": "Excluded File Relation Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#excluded-file-relation-validation"
                    },
                    {
                      "name": "GraphRegistry as Primary Registry",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#graphregistry-as-primary-registry",
                      "children": [
                        {
                          "name": "Requirements Processing Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#requirements-processing-specification"
                        }
                      ]
                    },
                    {
                      "name": "Integrated Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#integrated-validation",
                      "children": [
                        {
                          "name": "Two-Pass Validation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Internal Consistency Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#internal-consistency-validator"
                    },
                    {
                      "name": "Markdown Structure Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#markdown-structure-validator"
                    },
                    {
                      "name": "Relation Element Type Validator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#relation-element-type-validator"
                    },
                    {
                      "name": "Relation Type Validation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#relation-type-validation"
                    },
                    {
                      "name": "Two-Pass Validation Strategy",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#two-pass-validation-strategy"
                    },
                    {
                      "name": "Type Validation Error Requirement",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#type-validation-error-requirement"
                    },
                    {
                      "name": "Validate Cross-Component Dependencies",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-cross-component-dependencies"
                    },
                    {
                      "name": "Validate Filesystem Structure",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-filesystem-structure"
                    },
                    {
                      "name": "Validate Internal Consistency",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-internal-consistency"
                    },
                    {
                      "name": "Validate Markdown Structure",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-markdown-structure"
                    },
                    {
                      "name": "Validate Relation Types",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validate-relation-types",
                      "children": [
                        {
                          "name": "Relation Semantics Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#relation-semantics-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validation Error Handling",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Core/Validation.html#validation-error-handling",
                      "children": [
                        {
                          "name": "Error Message Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                        }
                      ]
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
                  "link": "requirements/Functional/Integration/CodeAlignment.html",
                  "children": [
                    {
                      "name": "BAT style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#bat-style-comment"
                    },
                    {
                      "name": "Code Traceability",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#code-traceability"
                    },
                    {
                      "name": "Comment Style by File Extension",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#comment-style-by-file-extension"
                    },
                    {
                      "name": "CSS style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#css-style-comment"
                    },
                    {
                      "name": "Dash style comment",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#dash-style-comment"
                    },
                    {
                      "name": "Slash style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#slash-style-comment"
                    },
                    {
                      "name": "SQL style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#sql-style-comment"
                    },
                    {
                      "name": "Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#traceability-format"
                    },
                    {
                      "name": "Validating Traceability Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#validating-traceability-format"
                    },
                    {
                      "name": "XML style comment",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/CodeAlignment.html#xml-style-comment"
                    }
                  ]
                },
                {
                  "name": "GitHubIntegration.md",
                  "type": "file",
                  "link": "requirements/Functional/Integration/GitHubIntegration.html",
                  "children": [
                    {
                      "name": "Automate Documentation Export",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automate-documentation-export"
                    },
                    {
                      "name": "Automate Pull Request Validations",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automate-pull-request-validations"
                    },
                    {
                      "name": "Automated Documentation Export on PR Merge",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#automated-documentation-export-on-pr-merge"
                    },
                    {
                      "name": "Generate Change Logs for Pull Requests",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Integration/GitHubIntegration.html#generate-change-logs-for-pull-requests"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Integration/Specifications.html",
                  "children": [
                    {
                      "name": "Comment Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Integration/Specifications.html#comment-style-specification"
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
                      "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html",
                      "children": [
                        {
                          "name": "Add Command Duplicate Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#add-command-duplicate-detection-test"
                        },
                        {
                          "name": "Add Command Error Messages Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#add-command-error-messages-test"
                        },
                        {
                          "name": "CLI Add Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-add-element-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "CLI Move Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-move-element-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "CLI Move File Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-move-file-test"
                        },
                        {
                          "name": "CLI Remove Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-remove-element-test"
                        },
                        {
                          "name": "CLI Rename Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#cli-rename-element-test"
                        },
                        {
                          "name": "Create Element Override Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#create-element-override-test",
                          "children": [
                            {
                              "name": "Create Element Override Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Create Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#create-element-test"
                        },
                        {
                          "name": "Delete Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#delete-element-test"
                        },
                        {
                          "name": "File Persistence Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#file-persistence-test",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Link Command Cross-Section Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#link-command-cross-section-detection-test"
                        },
                        {
                          "name": "Link Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#link-command-verification"
                        },
                        {
                          "name": "Merge Elements Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#merge-elements-test"
                        },
                        {
                          "name": "Move Element Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#move-element-test"
                        },
                        {
                          "name": "Move File Squash Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#move-file-squash-test"
                        },
                        {
                          "name": "Relation Consistency Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#relation-consistency-test"
                        },
                        {
                          "name": "Target Location Validation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#target-location-validation-test"
                        },
                        {
                          "name": "Unlink Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/ElementManipulationVerifications.html#unlink-command-verification"
                        }
                      ]
                    },
                    {
                      "name": "FormattingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html",
                      "children": [
                        {
                          "name": "Element Ordering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#element-ordering-verification",
                          "children": [
                            {
                              "name": "Element Ordering Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Format Command Requirements Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#format-command-requirements-verification"
                        },
                        {
                          "name": "Format Duplicate Removal Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#format-duplicate-removal-test"
                        },
                        {
                          "name": "Full Relations Insertion Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#full-relations-insertion-verification"
                        },
                        {
                          "name": "Relation Ordering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/FormattingVerifications.html#relation-ordering-verification"
                        }
                      ]
                    },
                    {
                      "name": "LintingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html",
                      "children": [
                        {
                          "name": "Lint Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html#lint-command-verification"
                        },
                        {
                          "name": "Redundant Hierarchical Attachment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Operations/Verifications/LintingVerifications.html#redundant-hierarchical-attachment-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Behaviors.html",
                  "children": [
                    {
                      "name": "Create Element Override Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                    },
                    {
                      "name": "Dry-Run Mode Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                    },
                    {
                      "name": "Element Ordering Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                    },
                    {
                      "name": "File Persistence Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                    },
                    {
                      "name": "Format Duplicate Removal Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#format-duplicate-removal-behavior"
                    },
                    {
                      "name": "Merge Content Transformation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Behaviors.html#merge-content-transformation-behavior"
                    }
                  ]
                },
                {
                  "name": "Constraints.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Constraints.html",
                  "children": [
                    {
                      "name": "Merge Type Compatibility Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Constraints.html#merge-type-compatibility-constraint"
                    },
                    {
                      "name": "Target Location Constraint",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                    }
                  ]
                },
                {
                  "name": "ElementManipulation.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/ElementManipulation.html",
                  "children": [
                    {
                      "name": "Create Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#create-element-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Delete Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#delete-element-operation"
                    },
                    {
                      "name": "Element Manipulation File Persistence",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#element-manipulation-file-persistence",
                      "children": [
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Merge Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#merge-element-operation"
                    },
                    {
                      "name": "Move Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#move-element-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Move File Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#move-file-operation",
                      "children": [
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        }
                      ]
                    },
                    {
                      "name": "Relation Consistency Maintenance",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#relation-consistency-maintenance"
                    },
                    {
                      "name": "Rename Element Operation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#rename-element-operation"
                    },
                    {
                      "name": "Target Location Validation and Auto-Creation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/ElementManipulation.html#target-location-validation-and-auto-creation",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Formatting.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Formatting.html",
                  "children": [
                    {
                      "name": "Document Structure Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#document-structure-normalization"
                    },
                    {
                      "name": "Element Ordering Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#element-ordering-normalization",
                      "children": [
                        {
                          "name": "Element Ordering Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#element-ordering-behavior"
                        }
                      ]
                    },
                    {
                      "name": "File Pattern Exclusion for Format",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#file-pattern-exclusion-for-format"
                    },
                    {
                      "name": "Format Consistency Enforcement",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#format-consistency-enforcement"
                    },
                    {
                      "name": "Format Duplicate Removal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#format-duplicate-removal"
                    },
                    {
                      "name": "Formatting Output",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#formatting-output"
                    },
                    {
                      "name": "Full Relations Insertion",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#full-relations-insertion",
                      "children": [
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        }
                      ]
                    },
                    {
                      "name": "Git-Style Diff Output for Format",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#git-style-diff-output-for-format"
                    },
                    {
                      "name": "Model Formatting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#model-formatting"
                    },
                    {
                      "name": "Relation Ordering Normalization",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#relation-ordering-normalization"
                    },
                    {
                      "name": "Replace Absolute Links with Relative Links",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Formatting.html#replace-absolute-links-with-relative-links"
                    }
                  ]
                },
                {
                  "name": "Linting.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Linting.html",
                  "children": [
                    {
                      "name": "Lint Auto-fix Capability",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#lint-auto-fix-capability",
                      "children": [
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Linting",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#model-linting"
                    },
                    {
                      "name": "Multi-Branch Convergence Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#multi-branch-convergence-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Hierarchical Attachment Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-hierarchical-attachment-detection"
                    },
                    {
                      "name": "Redundant Hierarchical Relations Detection and Auto-Removal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-hierarchical-relations-detection-and-auto-removal",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    },
                    {
                      "name": "Redundant Verify Relations Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Operations/Linting.html#redundant-verify-relations-detection",
                      "children": [
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Operations/Specifications.html",
                  "children": [
                    {
                      "name": "Create Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#create-element-workflow-specification"
                    },
                    {
                      "name": "Delete Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#delete-element-workflow-specification"
                    },
                    {
                      "name": "Document Structure Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#document-structure-specification"
                    },
                    {
                      "name": "Format Consistency Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#format-consistency-specification"
                    },
                    {
                      "name": "Lint Output Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#lint-output-specification"
                    },
                    {
                      "name": "Merge Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#merge-element-workflow-specification"
                    },
                    {
                      "name": "Move Element Workflow Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#move-element-workflow-specification"
                    },
                    {
                      "name": "Multi-Branch Convergence Detection Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#multi-branch-convergence-detection-specification"
                    },
                    {
                      "name": "Orphaned Children Error Message Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#orphaned-children-error-message-specification"
                    },
                    {
                      "name": "Redundant Hierarchical Relations Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#redundant-hierarchical-relations-specification"
                    },
                    {
                      "name": "Relation Operations Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-operations-specification"
                    },
                    {
                      "name": "Relation Ordering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-ordering-specification"
                    },
                    {
                      "name": "Relation Validation Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Operations/Specifications.html#relation-validation-specification"
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
                      "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.html"
                    },
                    {
                      "name": "SearchFiltering.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Output/DesignDocuments/SearchFiltering.html"
                    },
                    {
                      "name": "TraceFlowView.md",
                      "type": "design-document",
                      "link": "requirements/Functional/Output/DesignDocuments/TraceFlowView.html"
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
                      "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html",
                      "children": [
                        {
                          "name": "Automated Documentation Export on PR Merge Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#automated-documentation-export-on-pr-merge-verification"
                        },
                        {
                          "name": "Diagram Generation Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#diagram-generation-test"
                        },
                        {
                          "name": "Diagram Relation Filtering Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#diagram-relation-filtering-verification"
                        },
                        {
                          "name": "File Diagram Attachment Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#file-diagram-attachment-test"
                        },
                        {
                          "name": "Visualize Model Relationships Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/DiagramVerifications.html#visualize-model-relationships-verification"
                        }
                      ]
                    },
                    {
                      "name": "ReportingVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html",
                      "children": [
                        {
                          "name": "CLI Collect Command Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#cli-collect-command-test"
                        },
                        {
                          "name": "Containment Hierarchy Extraction Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-hierarchy-extraction-test"
                        },
                        {
                          "name": "Containment View Design Documents Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-design-documents-test"
                        },
                        {
                          "name": "Containment View JSON Output Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-json-output-test"
                        },
                        {
                          "name": "Containment View Mermaid Diagram Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-mermaid-diagram-test"
                        },
                        {
                          "name": "Containment View Text Output Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#containment-view-text-output-test"
                        },
                        {
                          "name": "Custom Element Type Tracking Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#custom-element-type-tracking-test"
                        },
                        {
                          "name": "HTML Export Containment View Integration Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#html-export-containment-view-integration-test"
                        },
                        {
                          "name": "Model Command Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#model-command-verification"
                        },
                        {
                          "name": "Multi-Type Search Filter Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#multi-type-search-filter-test"
                        },
                        {
                          "name": "Resources Report Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#resources-report-verification"
                        },
                        {
                          "name": "Reverse Model Traversal Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#reverse-model-traversal-test",
                          "children": [
                            {
                              "name": "Reverse Relation Traversal Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                            }
                          ]
                        },
                        {
                          "name": "Search Command Tests",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#search-command-tests"
                        },
                        {
                          "name": "Start Type Filter Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#start-type-filter-test",
                          "children": [
                            {
                              "name": "Start Element Type Filter Behavior",
                              "type": "attachment-element",
                              "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                            }
                          ]
                        },
                        {
                          "name": "TraceFlow View Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#traceflow-view-test"
                        },
                        {
                          "name": "Verification Coverage Report Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-coverage-report-test"
                        },
                        {
                          "name": "Verification Traces Filter Options Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-traces-filter-options-test"
                        },
                        {
                          "name": "Verification Traces From-Folder Test",
                          "type": "verification",
                          "link": "requirements/Functional/Output/Verifications/ReportingVerifications.html#verification-traces-from-folder-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Behaviors.html",
                  "children": [
                    {
                      "name": "Mermaid Diagram Interaction Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#mermaid-diagram-interaction-behavior"
                    },
                    {
                      "name": "Reverse Relation Traversal Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                    },
                    {
                      "name": "Short Mode Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                    },
                    {
                      "name": "Start Element Type Filter Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                    },
                    {
                      "name": "Verification Coverage Philosophy Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Behaviors.html#verification-coverage-philosophy-behavior"
                    }
                  ]
                },
                {
                  "name": "DiagramGeneration.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/DiagramGeneration.html",
                  "children": [
                    {
                      "name": "Diagram Generation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#diagram-generation",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "File Diagram Attachment Display",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#file-diagram-attachment-display",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "Interactive Mermaid Diagram Node Behavior",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#interactive-mermaid-diagram-node-behavior",
                      "children": [
                        {
                          "name": "Mermaid Interactive Features Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                        }
                      ]
                    },
                    {
                      "name": "Interactive Mermaid Diagrams",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#interactive-mermaid-diagrams",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "SysML-Compatible Relationship Rendering",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#sysml-compatible-relationship-rendering",
                      "children": [
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Trace Relation Non-Directional Behavior",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/DiagramGeneration.html#trace-relation-non-directional-behavior"
                    }
                  ]
                },
                {
                  "name": "Reporting.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Reporting.html",
                  "children": [
                    {
                      "name": "Collect Content from Requirement Chain",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#collect-content-from-requirement-chain",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Comma-Separated Type Filter Parsing",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#comma-separated-type-filter-parsing"
                    },
                    {
                      "name": "Containment View Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#containment-view-report",
                      "children": [
                        {
                          "name": "ContainmentView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.md"
                        },
                        {
                          "name": "Containment Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#containment-specification"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Resources Report Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Flexible Search Type Filtering",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#flexible-search-type-filtering"
                    },
                    {
                      "name": "Forward-Only Relation Traversal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#forward-only-relation-traversal",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Diagram Output Formats",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-diagram-output-formats",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Mermaid Diagram Generation Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                        },
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Reports",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-reports",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model Structure and Summaries",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#model-structure-and-summaries"
                    },
                    {
                      "name": "Provide Validation Reports",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#provide-validation-reports"
                    },
                    {
                      "name": "Resources Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#resources-report"
                    },
                    {
                      "name": "Reverse Relation Traversal",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#reverse-relation-traversal",
                      "children": [
                        {
                          "name": "Diagram Relation Filtering Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                        }
                      ]
                    },
                    {
                      "name": "Search Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#search-report-generator",
                      "children": [
                        {
                          "name": "SearchFiltering.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/SearchFiltering.md"
                        },
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        },
                        {
                          "name": "Resources Report Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Start Element Type Filtering",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#start-element-type-filtering"
                    },
                    {
                      "name": "TraceFlow View Report Generation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#traceflow-view-report-generation",
                      "children": [
                        {
                          "name": "TraceFlowView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/TraceFlowView.md"
                        },
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        },
                        {
                          "name": "Verification Roll-up Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                        },
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Tracing Structural Changes",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#tracing-structural-changes",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validation Report Generator",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#validation-report-generator",
                      "children": [
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    },
                    {
                      "name": "Verification Coverage Report",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Output/Reporting.html#verification-coverage-report",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Verification Coverage Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#verification-coverage-specification"
                        },
                        {
                          "name": "Verification Roll-up Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                        },
                        {
                          "name": "Verification Type Selection Guidelines",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                        },
                        {
                          "name": "Deterministic Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Output/Specifications.html",
                  "children": [
                    {
                      "name": "Collect Content Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#collect-content-specification"
                    },
                    {
                      "name": "Collect Output Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#collect-output-format-specification"
                    },
                    {
                      "name": "Color Scheme Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#color-scheme-specification"
                    },
                    {
                      "name": "Deterministic Output Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#deterministic-output-specification"
                    },
                    {
                      "name": "Diagram Relation Filtering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#diagram-relation-filtering-specification"
                    },
                    {
                      "name": "Diff Output Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                    },
                    {
                      "name": "Error Message Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                    },
                    {
                      "name": "JSON Output Structure",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                    },
                    {
                      "name": "Markdown Report Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#markdown-report-style-specification"
                    },
                    {
                      "name": "Mermaid Diagram Generation Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-generation-specification"
                    },
                    {
                      "name": "Mermaid Diagram Style Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                    },
                    {
                      "name": "Mermaid Interactive Features Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#mermaid-interactive-features-specification"
                    },
                    {
                      "name": "Resources Report Format Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#resources-report-format-specification"
                    },
                    {
                      "name": "SysML Rendering Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#sysml-rendering-specification"
                    },
                    {
                      "name": "Text Output Formatting",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                    },
                    {
                      "name": "Verification Trace Diagram Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Output/Specifications.html#verification-trace-diagram-specification"
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
                      "link": "requirements/Functional/Processing/DesignDocuments/ChangePropagation.html"
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
                      "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html",
                      "children": [
                        {
                          "name": "Change Impact Analysis Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-analysis-verification"
                        },
                        {
                          "name": "Change Impact Detection Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-detection-test"
                        },
                        {
                          "name": "Change Impact Relations Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-relations-test"
                        },
                        {
                          "name": "Change Impact Smart Filtering Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#change-impact-smart-filtering-test"
                        },
                        {
                          "name": "Element Content Extraction Test",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#element-content-extraction-test"
                        },
                        {
                          "name": "Structural Change Reports Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/ChangeImpactVerifications.html#structural-change-reports-verification"
                        }
                      ]
                    },
                    {
                      "name": "TraceVerifications.md",
                      "type": "file",
                      "link": "requirements/Functional/Processing/Verifications/TraceVerifications.html",
                      "children": [
                        {
                          "name": "Trace Relations No Cycles Verification",
                          "type": "verification",
                          "link": "requirements/Functional/Processing/Verifications/TraceVerifications.html#trace-relations-no-cycles-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/Behaviors.html",
                  "children": [
                    {
                      "name": "Change Propagation Behavior",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Behaviors.html#change-propagation-behavior"
                    }
                  ]
                },
                {
                  "name": "ChangeImpact.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/ChangeImpact.html",
                  "children": [
                    {
                      "name": "Change Impact Detection",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#change-impact-detection"
                    },
                    {
                      "name": "Requirements Change Propagation",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#requirements-change-propagation",
                      "children": [
                        {
                          "name": "ChangePropagation.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Processing/DesignDocuments/ChangePropagation.md"
                        }
                      ]
                    },
                    {
                      "name": "Structural Change Analyzer",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/ChangeImpact.html#structural-change-analyzer"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/Specifications.html",
                  "children": [
                    {
                      "name": "Verification Roll-up Specification",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Specifications.html#verification-roll-up-specification"
                    },
                    {
                      "name": "Verification Trace Tree Construction",
                      "type": "refinement",
                      "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                    }
                  ]
                },
                {
                  "name": "VerificationTraces.md",
                  "type": "file",
                  "link": "requirements/Functional/Processing/VerificationTraces.html",
                  "children": [
                    {
                      "name": "Verification Roll-up Strategy",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-roll-up-strategy"
                    },
                    {
                      "name": "Verification Trace Builder",
                      "type": "system-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-trace-builder"
                    },
                    {
                      "name": "Verification Upward Traceability",
                      "type": "user-requirement",
                      "link": "requirements/Functional/Processing/VerificationTraces.html#verification-upward-traceability",
                      "children": [
                        {
                          "name": "Traceability Reporting Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#traceability-reporting-specification"
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
          "name": "Interfaces",
          "type": "folder",
          "children": [
            {
              "name": "CLI",
              "type": "folder",
              "children": [
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "CLIVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html",
                      "children": [
                        {
                          "name": "CLI Git Commit Hash Flag Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#cli-git-commit-hash-flag-test"
                        },
                        {
                          "name": "CLI Help Structure Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#cli-help-structure-verification"
                        },
                        {
                          "name": "Verification Traces Element Navigation Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/CLI/Verifications/CLIVerifications.html#verification-traces-element-navigation-test"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Commands.md",
                  "type": "file",
                  "link": "requirements/Interfaces/CLI/Commands.html",
                  "children": [
                    {
                      "name": "Attachment Commands",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#attachment-commands",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "Attachment Input Auto-Detection Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#attachment-input-auto-detection-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Add Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-add-element-command",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        },
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Create Element Override Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#create-element-override-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Create Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#create-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Change Impact Report Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-change-impact-report-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Change Propagation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Behaviors.html#change-propagation-behavior"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Collect Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-collect-command",
                      "children": [
                        {
                          "name": "Collect Content Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#collect-content-specification"
                        },
                        {
                          "name": "Collect Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#collect-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Containment Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-containment-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Short Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "ContainmentView.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Output/DesignDocuments/ContainmentView.md"
                        },
                        {
                          "name": "D3.js Containment Tree Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Coverage Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-coverage-command",
                      "children": [
                        {
                          "name": "Verification Type Selection Guidelines",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#verification-type-selection-guidelines"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        }
                      ]
                    },
                    {
                      "name": "CLI Interface Structure",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-interface-structure"
                    },
                    {
                      "name": "CLI Lint Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-lint-command",
                      "children": [
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Lint Output Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#lint-output-specification"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Multi-Branch Convergence Detection Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#multi-branch-convergence-detection-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Merge Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-merge-element-command",
                      "children": [
                        {
                          "name": "Merge Content Transformation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#merge-content-transformation-behavior"
                        },
                        {
                          "name": "Merge Type Compatibility Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#merge-type-compatibility-constraint"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Merge Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#merge-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Model Diagram Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-model-diagram-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "Reverse Relation Traversal Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#reverse-relation-traversal-behavior"
                        },
                        {
                          "name": "Start Element Type Filter Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#start-element-type-filter-behavior"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move Asset Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-asset-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-element-command",
                      "children": [
                        {
                          "name": "Git Repository Scope Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Specifications.html#git-repository-scope-specification"
                        },
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Move Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#move-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Move File Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-move-file-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Target Location Constraint",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Constraints.html#target-location-constraint"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "CLI Remove Asset Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-remove-asset-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Remove Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-remove-element-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Delete Element Workflow Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#delete-element-workflow-specification"
                        }
                      ]
                    },
                    {
                      "name": "CLI Rename Element Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-rename-element-command",
                      "children": [
                        {
                          "name": "File Persistence Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#file-persistence-behavior"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "CLI Resources Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-resources-command",
                      "children": [
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        }
                      ]
                    },
                    {
                      "name": "CLI Search Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-search-command",
                      "children": [
                        {
                          "name": "Supported Element Types Specification",
                          "type": "attachment-element",
                          "link": "requirements/Refinements.html#supported-element-types-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Short Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Behaviors.html#short-mode-behavior"
                        },
                        {
                          "name": "Text Output Formatting",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#text-output-formatting"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "CLI Traces Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#cli-traces-command",
                      "children": [
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Verification Trace Tree Construction",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Processing/Specifications.html#verification-trace-tree-construction"
                        },
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "Type Validation Error Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#type-validation-error-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Detailed Error Handling and Logging",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#detailed-error-handling-and-logging",
                      "children": [
                        {
                          "name": "Validation Error Reporting Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Format Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#format-command",
                      "children": [
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        }
                      ]
                    },
                    {
                      "name": "Relation Commands",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#relation-commands",
                      "children": [
                        {
                          "name": "Relation Operations Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Specifications.html#relation-operations-specification"
                        },
                        {
                          "name": "RelationTypes.md",
                          "type": "attachment-file",
                          "link": "requirements/Functional/Core/DesignDocuments/RelationTypes.md"
                        },
                        {
                          "name": "Dry-Run Mode Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Operations/Behaviors.html#dry-run-mode-behavior"
                        },
                        {
                          "name": "Diff Output Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#diff-output-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Validate Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#validate-command",
                      "children": [
                        {
                          "name": "Two-Pass Validation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#two-pass-validation-behavior"
                        },
                        {
                          "name": "Validation Error Reporting Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Core/Behaviors.html#validation-error-reporting-behavior"
                        },
                        {
                          "name": "JSON Output Structure",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#json-output-structure"
                        },
                        {
                          "name": "Error Message Format Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#error-message-format-specification"
                        }
                      ]
                    },
                    {
                      "name": "Verification Traces Element Navigation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/CLI/Commands.html#verification-traces-element-navigation"
                    }
                  ]
                }
              ]
            },
            {
              "name": "WebInterface",
              "type": "folder",
              "children": [
                {
                  "name": "Verifications",
                  "type": "folder",
                  "children": [
                    {
                      "name": "HTMLGenerationVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html",
                      "children": [
                        {
                          "name": "Component Reuse Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#component-reuse-verification"
                        },
                        {
                          "name": "HTML Validity Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#html-validity-verification"
                        },
                        {
                          "name": "Integration Test Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#integration-test-verification"
                        },
                        {
                          "name": "Mobile Responsiveness Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#mobile-responsiveness-verification"
                        },
                        {
                          "name": "Responsive Design Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/HTMLGenerationVerifications.html#responsive-design-verification"
                        }
                      ]
                    },
                    {
                      "name": "WebInterfaceVerifications.md",
                      "type": "file",
                      "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html",
                      "children": [
                        {
                          "name": "Attachment Export Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#attachment-export-verification"
                        },
                        {
                          "name": "Containment Attachment Links Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#containment-attachment-links-verification"
                        },
                        {
                          "name": "Diagram Attachment Display Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#diagram-attachment-display-verification"
                        },
                        {
                          "name": "HTML Export Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#html-export-verification"
                        },
                        {
                          "name": "Model View Element Navigation Test",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#model-view-element-navigation-test"
                        },
                        {
                          "name": "Serve Command Verification",
                          "type": "verification",
                          "link": "requirements/Interfaces/WebInterface/Verifications/WebInterfaceVerifications.html#serve-command-verification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "Behaviors.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Behaviors.html",
                  "children": [
                    {
                      "name": "Web Interface Navigation Behavior",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Behaviors.html#web-interface-navigation-behavior"
                    }
                  ]
                },
                {
                  "name": "Features.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Features.html",
                  "children": [
                    {
                      "name": "Attachment Export",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#attachment-export"
                    },
                    {
                      "name": "Containment View Attachment Links",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#containment-view-attachment-links",
                      "children": [
                        {
                          "name": "D3.js Containment Tree Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                        }
                      ]
                    },
                    {
                      "name": "Diagram Attachment Display",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#diagram-attachment-display",
                      "children": [
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        }
                      ]
                    },
                    {
                      "name": "HTML Export",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#html-export",
                      "children": [
                        {
                          "name": "Web Interface Navigation Behavior",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Behaviors.html#web-interface-navigation-behavior"
                        }
                      ]
                    },
                    {
                      "name": "Model-Centric View Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#model-centric-view-generation",
                      "children": [
                        {
                          "name": "Mermaid Diagram Style Specification",
                          "type": "attachment-element",
                          "link": "requirements/Functional/Output/Specifications.html#mermaid-diagram-style-specification"
                        },
                        {
                          "name": "HTML Export Pipeline Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                        },
                        {
                          "name": "HTML Navigation Bar Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                        }
                      ]
                    },
                    {
                      "name": "Model View Element Navigation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#model-view-element-navigation"
                    },
                    {
                      "name": "Serve Command",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#serve-command",
                      "children": [
                        {
                          "name": "HTML Export Pipeline Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                        }
                      ]
                    },
                    {
                      "name": "Web Interface Color Scheme",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/Features.html#web-interface-color-scheme",
                      "children": [
                        {
                          "name": "HTML Navigation Bar Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                        },
                        {
                          "name": "HTML Branding Specification",
                          "type": "attachment-element",
                          "link": "requirements/Interfaces/WebInterface/Specifications.html#html-branding-specification"
                        }
                      ]
                    }
                  ]
                },
                {
                  "name": "HTMLGeneration.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html",
                  "children": [
                    {
                      "name": "Component-Based HTML Architecture",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#component-based-html-architecture"
                    },
                    {
                      "name": "CSS Framework Integration",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#css-framework-integration"
                    },
                    {
                      "name": "Mobile-Friendly Documentation",
                      "type": "user-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#mobile-friendly-documentation"
                    },
                    {
                      "name": "Responsive HTML Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#responsive-html-generation"
                    },
                    {
                      "name": "Type-Safe HTML Generation",
                      "type": "system-requirement",
                      "link": "requirements/Interfaces/WebInterface/HTMLGeneration.html#type-safe-html-generation"
                    }
                  ]
                },
                {
                  "name": "Specifications.md",
                  "type": "file",
                  "link": "requirements/Interfaces/WebInterface/Specifications.html",
                  "children": [
                    {
                      "name": "D3.js Containment Tree Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#d3js-containment-tree-specification"
                    },
                    {
                      "name": "HTML Branding Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-branding-specification"
                    },
                    {
                      "name": "HTML Export Pipeline Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-export-pipeline-specification"
                    },
                    {
                      "name": "HTML Navigation Bar Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#html-navigation-bar-specification"
                    },
                    {
                      "name": "Web Interface Style Specification",
                      "type": "refinement",
                      "link": "requirements/Interfaces/WebInterface/Specifications.html#web-interface-style-specification"
                    }
                  ]
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
            }
          ]
        },
        {
          "name": "Refinements.md",
          "type": "file",
          "link": "requirements/Refinements.html",
          "children": [
            {
              "name": "Containment Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#containment-specification"
            },
            {
              "name": "Refinement Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#refinement-specification"
            },
            {
              "name": "Relation Semantics Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#relation-semantics-specification"
            },
            {
              "name": "Supported Element Types Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#supported-element-types-specification"
            },
            {
              "name": "Traceability Reporting Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#traceability-reporting-specification"
            },
            {
              "name": "Verification Coverage Specification",
              "type": "refinement",
              "link": "requirements/Refinements.html#verification-coverage-specification"
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
              "name": "Defining Model Structure",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#defining-model-structure"
            },
            {
              "name": "Formatting Model Documents",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#formatting-model-documents"
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
              "name": "Linting Model Quality",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#linting-model-quality"
            },
            {
              "name": "Operating on Model Elements",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#operating-on-model-elements"
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
              "name": "System Model Interfaces",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#system-model-interfaces"
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
            },
            {
              "name": "Verification Traceability",
              "type": "user-requirement",
              "link": "requirements/UserStories.html#verification-traceability"
            }
          ]
        }
      ]
    }
  ]
}
```


</div>

<script>
// Hide icicle view after page loads (both render first so D3 can calculate dimensions)
document.addEventListener('DOMContentLoaded', function() {
    document.getElementById('view-icicle').style.display = 'none';
});

function showView(view) {
    // Hide all views
    document.querySelectorAll('.containment-view').forEach(el => el.style.display = 'none');
    // Remove active from all buttons
    document.querySelectorAll('.view-btn').forEach(btn => btn.classList.remove('active'));
    // Show selected view
    document.getElementById('view-' + view).style.display = 'block';
    // Mark button as active
    document.getElementById('btn-' + view).classList.add('active');
}
</script>

<style>
.view-toggle {
    display: flex;
    gap: 8px;
    margin-bottom: 16px;
}
.view-btn {
    padding: 8px 20px;
    border: 1px solid #BDBDBD;
    background: #fff;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
}
.view-btn:hover {
    background: #F5F5F5;
}
.view-btn.active {
    background: var(--color-primary, #3F51B5);
    color: #fff;
    border-color: var(--color-primary, #3F51B5);
}
.view-instructions {
    color: #757575;
    font-size: 13px;
    margin: 0 0 12px 0;
    font-style: italic;
}
</style>
