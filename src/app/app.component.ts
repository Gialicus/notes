import { Component, ElementRef, OnInit, ViewChild } from "@angular/core";
import { Observable, of } from "rxjs";
import Note from "./interfaces/note";
import invokeFactory from "./utils/invoke_pipe";
import { Cmd, RustEvent } from "./interfaces/cmd";
import { appWindow } from "@tauri-apps/api/window";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent implements OnInit {
  @ViewChild("thema") thema?: ElementRef;
  constructor() {}
  ngOnInit(): void {
    appWindow.listen(RustEvent.MENU_SELECTED, (event) => {
      this.thema?.nativeElement.setAttribute("data-theme", event.payload);
    });
  }
  addNote(event: Note): void {
    invokeFactory(Cmd.ADD_NOTE, { ...event }).subscribe();
  }
}
