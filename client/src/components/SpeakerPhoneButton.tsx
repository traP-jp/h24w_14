import { AudioOutlined } from "@ant-design/icons";
import { Button } from "antd";
import React from "react";

export const StampPicker: React.FC = () => {
  const clickHandler = () => {
    console.log("stamp clicked");
  };
  return <Button icon={<AudioOutlined />} onClick={clickHandler} />;
};
