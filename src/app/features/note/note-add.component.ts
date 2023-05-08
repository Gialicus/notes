import { Component } from "@angular/core";
import { Router } from "@angular/router";
import { Cmd } from "src/app/interfaces/cmd";
import { Note } from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";

@Component({
  selector: "app-note-add",
  templateUrl: "./note-add.component.html",
  styles: [],
})
export class NoteAddComponent {
  constructor(private router: Router) {}
  addNote(event: Note): void {
    invokeFactory(Cmd.ADD_NOTE, {
      title: event.title,
      body: event.body,
    }).subscribe(() => this.router.navigateByUrl("/notes/list"));
  }
}
