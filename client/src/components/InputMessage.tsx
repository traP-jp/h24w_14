import { SendOutlined } from "@ant-design/icons";
import { Button, Input } from "antd";
import { useAtomValue } from "jotai";
import React, { useCallback, useState } from "react";
import { useCreateMessage } from "../api/message";
import { roundedUserPositionAtom } from "../state/userPosition";
const { TextArea } = Input;

export const InputMessage: React.FC = () => {
  const [message, setMessage] = useState("");
  const [isTextAreaFocused, setIsTextAreaFocused] = useState(false);
  const position = useAtomValue(roundedUserPositionAtom);
  const { trigger } = useCreateMessage();
  const [isSending, setIsSending] = useState(false);

  const sendMessage = useCallback(async () => {
    if (message === "") return;
    if (isSending) return;

    setIsSending(true);

    await trigger({ content: message, position: position ?? undefined });
    setMessage("");

    setIsSending(false);
  }, [isSending, message, position, trigger]);

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
      <Button
        className="size-6 mb-1"
        onClick={sendMessage}
        disabled={isSending}
        icon={<SendOutlined />}
      />
    </div>
  );
};
