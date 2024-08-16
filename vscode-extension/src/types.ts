export type Reminder = {
  datetime: number;
  message: string;
  position: {
    file: string;
    line: number;
  };
  meta: Record<string, string>;
};

export type ListCommandOutput = {
  expired: Reminder[];
  upcoming: Reminder[];
};
