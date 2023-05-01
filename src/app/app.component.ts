import { Component, ElementRef, OnInit, ViewChild } from "@angular/core";
import { Observable, from, of } from "rxjs";
import Note from "./interfaces/note";
import invokeFactory from "./utils/invoke_pipe";
import { Cmd } from "./interfaces/cmd";
import { appWindow } from "@tauri-apps/api/window";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent implements OnInit {
  @ViewChild("thema") thema?: ElementRef;
  list: Observable<Note[]> = of([] as Note[]);
  constructor() {}
  ngOnInit(): void {
    this.list = invokeFactory(Cmd.BROWSE_NOTE);
    appWindow.listen("menu-selected", (event) => {
      this.thema?.nativeElement.setAttribute("data-theme", event.payload);
    });
  }
  addNote(event: SubmitEvent, title: string, body: string): void {
    event.preventDefault();
    invokeFactory(Cmd.ADD_NOTE, { title, body }).subscribe();
  }
}
