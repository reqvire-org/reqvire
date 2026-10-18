

export const fixture = {"title":"Content Renderer Frames","description":"Renderer shell patterns use already-rendered mock content instead of app renderers."};
const CODE_SAMPLE = `export function coverageFor(requirement) {
  const verifications = requirement.relations.verifiedBy;
  return verifications.length > 0 ? "covered" : "gap";
}`;

export { CODE_SAMPLE };
