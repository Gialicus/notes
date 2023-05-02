import { Component, EventEmitter, Output } from "@angular/core";
import { FormBuilder, FormControl, FormGroup } from "@angular/forms";
import Note from "src/app/interfaces/note";

@Component({
  selector: "app-note-form",
  templateUrl: "./note-form.component.html",
  styles: [],
})
export class NoteFormComponent {
  form: FormGroup;
  @Output() submit: EventEmitter<Note> = new EventEmitter();
  constructor(private fb: FormBuilder) {
    this.form = this.fb.group({
      title: new FormControl("", []),
      body: new FormControl("", []),
    });
  }
  onSubmit() {
    this.submit.emit(this.form.value);
  }
}
