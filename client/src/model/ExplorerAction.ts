export interface ExplorerAction extends Explorer {
  type: "arrive" | "move" | "leave";
}

interface Explorer {
  id: string;
  inner: {
    id: string;
    name: string;
    displayName: string;
    createdAt: string;
    updatedAt: string;
  };
  position: {
    x: number;
    y: number;
  };
}
