import { SmileOutlined } from "@ant-design/icons";
import { Button, Popover } from "antd";
import React, { useState } from "react";

export const StampPicker: React.FC = () => {
  const [isPopoverOpen, setPopoverOpen] = useState(false);
  const icons = Array(20).fill(<SmileOutlined />);

  return (
    <Popover
      content={
        <div className="grid grid-cols-4 gap-2">
          {icons.map((icon, index) => (
            <Button
              key={index}
              type="text"
              icon={icon}
              className="hover:bg-background-secondary"
            />
          ))}
        </div>
      }
      trigger="click"
      open={isPopoverOpen}
      onOpenChange={setPopoverOpen}
    >
      <Button icon={<SmileOutlined />} />
    </Popover>
  );
};
