# Initialization Command Verifications

This document verifies the requirements for the ReqFlow initialization command.

## Initialization


### Basic Initialization Test

The verification test checks that a new ReqFlow project can be initialized with the correct directory structure and configuration file.

#### Metadata
  * type: verification

#### Acceptance Criteria
- Initialize a new project in an empty directory
- Project structure should be created successfully
- Configuration file (reqflow.yaml) should be created
- Specifications directory structure should be created

#### Test Criteria
- Command exits with success (0) return code
- Expected directories and files exist

#### Test Procedure
1. Create a temporary empty directory
2. Run the ReqFlow init command
3. Verify that the command exits with success (0) return code
4. Verify that the expected directories are created:
   - specifications/
   - specifications/SystemRequirements/
   - specifications/DesignSpecifications/
   - specifications/Verifications/
5. Verify that a reqflow.yaml configuration file is created

#### Relations
  * verify: [../SystemRequirements/Requirements.md#Initialization Command](../SystemRequirements/Requirements.md#initialization-command)
  * trace: [tests/e2e-init/test_init_success.sh](/tests/e2e-init/test_init_success.sh)

---

### Existing Configuration Detection Test (YAML)

The verification test checks that the init command fails when a reqflow.yaml file already exists.

#### Metadata
  * type: verification

#### Acceptance Criteria
- Attempt to initialize a project in a directory with existing reqflow.yaml
- Initialization should fail with an error message
- No changes should be made to the existing structure

#### Test Criteria
- Command exits with error (non-zero) return code
- Error message mentions existing configuration file
- Original reqflow.yaml content remains unchanged

#### Test Procedure
1. Create a temporary directory with a dummy reqflow.yaml file
2. Run the ReqFlow init command
3. Verify that the command exits with an error (non-zero) return code
4. Verify that an error message is displayed mentioning the existing configuration file
5. Verify that the original reqflow.yaml file is not modified

#### Implementation
- Test is implemented in `/tests/e2e-init/test_init_existing_yaml.sh`

#### Relations
  * verify: [../SystemRequirements/Requirements.md#Initialization Command Configuration Check](../SystemRequirements/Requirements.md#initialization-command-configuration-check)
  * trace: [tests/e2e-init/test_init_existing_yaml.sh](/tests/e2e-init/test_init_existing_yaml.sh)

---

### Existing Configuration Detection Test (YML)

The verification test checks that the init command fails when a reqflow.yml file already exists.

#### Metadata
  * type: verification

#### Acceptance Criteria
- Attempt to initialize a project in a directory with existing reqflow.yml
- Initialization should fail with an error message
- No changes should be made to the existing structure

#### Test Criteria
- Command exits with error (non-zero) return code
- Error message mentions existing configuration file
- Original reqflow.yml content remains unchanged

#### Test Procedure
1. Create a temporary directory with a dummy reqflow.yml file
2. Run the ReqFlow init command
3. Verify that the command exits with an error (non-zero) return code
4. Verify that an error message is displayed mentioning the existing configuration file
5. Verify that the original reqflow.yml file is not modified

#### Relations
  * verify: [../SystemRequirements/Requirements.md#Initialization Command Configuration Check](../SystemRequirements/Requirements.md#initialization-command-configuration-check)
  * trace: [tests/e2e-init/test_init_existing_yml.sh](/tests/e2e-init/test_init_existing_yml.sh)

---

### Alternative Configuration Format Test

The verification test checks that the init command succeeds when other configuration formats (like .reqflow.yml) exist.

#### Metadata
* type: verification

#### Acceptance Criteria
- Initialize a project in a directory with existing .reqflow.yml
- Initialization should succeed despite the hidden config file
- Project structure should be created successfully

#### Test Criteria
- Command exits with success (0) return code
- Expected directories and files exist
- Original .reqflow.yml file remains untouched
- New reqflow.yaml is created

#### Test Procedure
1. Create a temporary directory with a dummy .reqflow.yml file
2. Run the ReqFlow init command
3. Verify that the command exits with success (0) return code
4. Verify that the expected directories and files are created
5. Verify that the original .reqflow.yml file is not modified

#### Relations
  * verify: [../SystemRequirements/Requirements.md#Initialization Command Configuration Check](../SystemRequirements/Requirements.md#initialization-command-configuration-check)
  * trace: [tests/e2e-init/test_init_dot_reqflow.sh](/tests/e2e-init/test_init_dot_reqflow.sh)
