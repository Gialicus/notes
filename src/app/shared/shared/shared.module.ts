import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { ReactiveFormsModule } from "@angular/forms";
import { NoteFormComponent } from "../form/note-form.component";

const COMPONENTS = [NoteFormComponent];

@NgModule({
  declarations: COMPONENTS,
  imports: [CommonModule, ReactiveFormsModule],
  exports: COMPONENTS,
})
export class SharedModule {}
