

export const fixture = {"title":"ElementIcon","description":"Colored model-element marker. Contracts use diamond subtype glyphs; verification subtypes use verification glyphs."};

const ELEMENT_ROLES = [
    "capability", "requirement", "contract", "source", "constraint",
    "behavior", "state", "input-output", "specification", "semantic-contract", "semantic-query",
    "verification-objective", "test-verification", "formal-proof-verification",
    "analysis-verification", "inspection-verification", "demonstration-verification",
    "verification", "ontology", "resource",
] as const;
export { ELEMENT_ROLES };
