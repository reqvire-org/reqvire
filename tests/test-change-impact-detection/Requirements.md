# System Requirements

This file should be processed.

## Implementation Elements

### Power Control

Power control.

### CPU Manager

CPU Manager.


### Battery Manager

Battery manager.


### Network Manager

Network manager.



## Verification Elements

### Power Saving

Power saving.

#### Metadata
  * type: verification

### CPU Throttling

Cpu Throttling

#### Metadata
  * type: verification


### Screen Brightness

Screen brightness.

#### Metadata
  * type: verification

### Battery Saving

Battery Saving.

#### Metadata
  * type: verification



## Requirements

### System Power Management

Main req.

---

### Power Saving Mode

The systsem shall activate power-saving mode when the battery level drops below 20%.  

#### Relations
  * refine: [System Power Management](#system-power-management)
  * satisfiedBy: [software/power_control.md](software/power_control.md)
  * verifiedBy: [test_cases/power_saving](#power-saving)

---

### CPU Power Reduction

The system shall reduce CPU frequency by 30% in power-saving mode.  

#### Relations
  * derivedFrom: [Power Saving Mode](#power-saving-mode)
  * satisfiedBy: [software/cpu_manager.md](software/cpu_manager.md)
  * verifiedBy: [test_cases/cpu_throttling](#cpu-throttling)

---

### Screen Brightness Adjustment

The system shall reduce screen brightness by 40% in power-saving mode.  

#### Relations
  * derivedFrom: [Power Saving Mode](#power-saving-mode)
  * verifiedBy: [test_cases/screen_brightness](#screen-brightness)

---

### Battery Optimization

The system shall disable non-essential background services when battery levels drop below 15%.  

#### Relations
  * derivedFrom: [System Power Management](#system-power-management)
  * satisfiedBy: [software/battery_manager.md](software/battery_manager.md)
  * verifiedBy: [test_cases/battery_saving](#battery-saving)

---

### Network Power Optimization
The system shall reduce network polling frequency when battery levels drop below 15%.  

#### Relations
  * derivedFrom: [Battery Optimization](#battery-optimization)

