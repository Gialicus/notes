import { Component, Inject, OnInit, Renderer2 } from "@angular/core";
import { RustEvent } from "./interfaces/cmd";
import { appWindow } from "@tauri-apps/api/window";
import { DOCUMENT } from "@angular/common";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent implements OnInit {
  constructor(
    @Inject(DOCUMENT) private document: Document,
    private renderer: Renderer2
  ) {}
  ngOnInit(): void {
    appWindow.listen(RustEvent.MENU_SELECTED, (event) => {
      this.renderer.setAttribute(
        this.document.body,
        "data-theme",
        event.payload as string
      );
    });
  }
}
