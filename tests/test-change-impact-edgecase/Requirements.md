# Elements


### Test Capability Test Change Impact Edgecase Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Deployments

Root requiement.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-change-impact-edgecase-requirements-md)
---

### Deployment Plugins

The system shall allow authorized users in EnvironmentsArea to add, remove, and configure plugins for deployments.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-change-impact-edgecase-requirements-md)
  * derivedFrom: [Deployments](#deployments)
---

### Add Plugin

The system shall allow authorized users in EnvironmentsArea to add plugins to enhance deployment functionalities.

#### Details
 * The maximum number of plugins that can be associated with a deployment is **5**.
 * The maximum size for each plugin that can be uploaded or managed by a user is **104,857,600 bytes (100 MB)**.
 * The plugin should be added to all nodes part of the deployment.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-change-impact-edgecase-requirements-md)
  * verifiedBy: [Validate Plugin Management](Verifications.md#validate-plugin-management)
  * derivedFrom: [Deployment Plugins](#deployment-plugins)
---

### Remove Plugins

The system shall provide users the ability to remove plugins from deployments when they are no longer required.

#### Details
 * Only authorized users can remove a plugin.
 * The plugin should be removied from all nodes part of the deployment.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-change-impact-edgecase-requirements-md)
  * verifiedBy: [Validate Plugin Removal](Verifications.md#validate-plugin-removal)
  * derivedFrom: [Deployment Plugins](#deployment-plugins)
