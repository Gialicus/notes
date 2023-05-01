import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import { Observable, from, map, of } from "rxjs";
import Note from "./interfaces/note";
import invokeFactory from "./utils/invoke_pipe";
import { Cmd } from "./interfaces/cmd";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [],
})
export class AppComponent implements OnInit {
  list: Observable<Note[]> = of([] as Note[]);
  ngOnInit(): void {
    this.list = invokeFactory(Cmd.BROWSE_NOTE);
  }
  addNote(event: SubmitEvent, title: string, body: string): void {
    event.preventDefault();
    invokeFactory(Cmd.ADD_NOTE, { title, body }).subscribe();
  }
}
