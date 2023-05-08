export interface Note {
  id: string;
  title: string;
  body: string;
}

export interface UpdateNote {
  id: string;
  body: Partial<Note>;
}
