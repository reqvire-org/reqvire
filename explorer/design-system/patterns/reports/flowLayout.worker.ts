// ELK's dedicated browser-worker entry installs its message dispatcher here.
// Keep the engine out of the main thread and let elk-api own its wire protocol.
import "elkjs/lib/elk-worker.min.js";
