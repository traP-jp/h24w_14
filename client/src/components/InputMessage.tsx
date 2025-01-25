import { SendOutlined } from "@ant-design/icons";
import { Input } from "antd";
import React, { useCallback, useState } from "react";
const { TextArea } = Input;

export const InputMessage: React.FC = () => {
  const [message, setMessage] = useState("");
  const [isTextAreaFocused, setIsTextAreaFocused] = useState(false);

  const sendMessage = useCallback(() => {
    setMessage("");
  }, []);

  const handleInputChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      setMessage(e.target.value);
    },
    [],
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
      if (isTextAreaFocused && (e.metaKey || e.ctrlKey) && e.key === "Enter") {
        e.preventDefault();
        sendMessage();
      }
    },
    [isTextAreaFocused, sendMessage],
  );

  const handleFocus = useCallback(() => {
    setIsTextAreaFocused(true);
  }, []);

  const handleBlur = useCallback(() => {
    setIsTextAreaFocused(false);
  }, []);

  return (
    <div className="flex items-end gap-1 p-2 bg-background-secondary">
      <TextArea
        placeholder="メッセージを入力"
        autoSize={{ minRows: 1, maxRows: 9 }}
        value={message}
        onChange={handleInputChange}
        onFocus={handleFocus}
        onBlur={handleBlur}
        onKeyDown={handleKeyDown}
      />
      <button className="size-6 mb-1" onClick={sendMessage} type="button">
        <SendOutlined role="img" aria-label="Send Message" />
      </button>
    </div>
  );
};
