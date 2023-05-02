import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { SharedModule } from "src/app/shared/shared/shared.module";
import { NoteListComponent } from "./note-list.component";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { NoteAddComponent } from "./note-add.component";
const COMPONENTS = [NoteListComponent, NoteAddComponent];

@NgModule({
  declarations: COMPONENTS,
  imports: [CommonModule, SharedModule, FontAwesomeModule],
  exports: COMPONENTS,
})
export class NoteModule {}
