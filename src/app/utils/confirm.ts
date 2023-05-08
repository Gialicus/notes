import { from } from "rxjs";

export default function confirmPipe(msg: string) {
  const promise = confirm(msg) as unknown as Promise<boolean>;
  return from(promise);
}
