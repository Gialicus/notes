import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";
import { catchError, from, map, throwError } from "rxjs";

export default function invokeFactory<T>(
  cmd: string,
  args?: InvokeArgs | undefined
) {
  return from(invoke<string>(cmd, args)).pipe(
    map((v) => JSON.parse(v) as T),
    catchError((e) => {
      console.error(e);
      return throwError(() => e);
    })
  );
}
