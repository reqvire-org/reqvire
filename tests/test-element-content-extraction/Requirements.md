# System Requirements

This file should be processed.

## Requirements

### REQ 0

Root requirement for relations to work.

#### Metadata
* type: user-requirement

### REQ 1

This is simple requirement with main text only.

#### Metadata
* type: user-requirement

### REQ 1A

This is simple requirement with main text and details.

#### Metadata
* type: user-requirement

#### Details

REQ 1A details.


### REQ 2

Requirement with main text and relations.

#### Relations
  * derivedFrom: #req-0
  
  
### REQ 3

Requirement with main text and relations and metadata.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #req-0
  
  
### REQ 4

Requirement with main text and relations and metadata and details.

#### Details

REQ 4 Details.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: #req-0
  
  
### REQ 5

Requirement with main text and relations and metadata and details different order (A).

#### Metadata
  * type: requirement

#### Details

REQ 5 Details.

#### Relations
  * derivedFrom: #req-0
  
    
  
### REQ 6

Requirement with main text and relations and metadata and details different order (B).


#### Relations
  * derivedFrom: #req-0
  
 
#### Metadata
  * type: requirement

#### Details

REQ 6 Details.

  
  
### REQ 7

Requirement with main text and relations and metadata and details different order (C).


#### Relations
  * derivedFrom: #req-0
  
#### Details

REQ 7 Details.

 
#### Metadata
  * type: requirement



### REQ 8

Requirement with main text and relations details with <details> element that should not break parsing and validation.

  
#### Details

<details>
### REQ 8

Nested requirement which should not be processed as requirement.


#### Relations
  * derivedFrom: #req-0

        
</details>


#### Relations
  * derivedFrom: #req-0

        
      
  
    
  
  
  
