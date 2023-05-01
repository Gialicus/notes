import { Injectable, Output } from "@angular/core";
import { BehaviorSubject, Subject, map, of } from "rxjs";

@Injectable({
  providedIn: "root",
})
export class ThemeSelectorService {
  private _themes: Record<number, string> = {
    0: "light",
    1: "dark",
    2: "cupcake",
    3: "bumblebee",
    4: "emerald",
    5: "corporate",
    6: "synthwave",
    7: "retro",
    8: "cyberpunk",
    9: "valentine",
    10: "halloween",
    11: "garden",
    12: "forest",
    13: "aqua",
    14: "lofi",
    15: "pastel",
    16: "fantasy",
    17: "wireframe",
    18: "black",
    19: "luxury",
    20: "dracula",
    21: "cmyk",
    22: "autumn",
    23: "business",
    24: "acid",
    25: "lemonade",
    26: "night",
    27: "coffee",
    28: "winter",
  };
  themeSub = new BehaviorSubject(8);
  set theme(index: number) {
    this.themeSub.next(index);
  }
  get theme$() {
    return this.themeSub.asObservable().pipe(map((th) => this._themes[th]));
  }
  get themes$() {
    return of(this._themes).pipe(map((rec) => Object.entries(rec)));
  }
}
