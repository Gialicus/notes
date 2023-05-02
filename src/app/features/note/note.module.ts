import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { SharedModule } from "src/app/shared/shared/shared.module";
import { NoteListComponent } from "./note-list.component";

const COMPONENTS = [NoteListComponent];

@NgModule({
  declarations: COMPONENTS,
  imports: [CommonModule, SharedModule],
  exports: COMPONENTS,
})
export class NoteModule {}
