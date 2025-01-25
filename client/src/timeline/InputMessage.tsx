import { SendOutlined } from "@ant-design/icons";
import { Input } from "antd";
import React from "react";
const { TextArea } = Input;

export const InputMessage: React.FC = () => {
  return (
    <div className="flex items-end gap-2 p-2 bg-background-secondary">
      <TextArea autoSize={true} />
      <SendOutlined className="size-6 mb-1" />
    </div>
  );
};
