import child_process from "node:child_process";
import util from "node:util";

export const execFile = util.promisify(child_process.execFile);
