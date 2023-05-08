import {
  Component,
  EventEmitter,
  Input,
  OnChanges,
  Output,
} from "@angular/core";
import { FormBuilder, FormControl, FormGroup } from "@angular/forms";
import { Note } from "src/app/interfaces/note";

@Component({
  selector: "app-note-form",
  templateUrl: "./note-form.component.html",
  styles: [],
})
export class NoteFormComponent implements OnChanges {
  form: FormGroup;
  @Input() note: Note | null = null;
  @Output() save: EventEmitter<Note> = new EventEmitter();
  constructor(private fb: FormBuilder) {
    this.form = this.fb.group({
      title: new FormControl("", []),
      body: new FormControl("", []),
    });
  }
  ngOnChanges(): void {
    if (!this.note) return;
    this.form.get("title")?.patchValue(this.note?.title);
    this.form.get("body")?.patchValue(this.note?.body);
  }
  onSubmit() {
    this.save.emit(this.form.value);
  }
}
