import { so1ve } from "@so1ve/eslint-config";

export default so1ve({
	ignores: ["src-tauri/target", "src-tauri/gen", "src-tauri/Cargo.lock"],
});
