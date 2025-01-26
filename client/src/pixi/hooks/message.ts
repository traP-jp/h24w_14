import { useCallback, useState } from "react";
import { Message } from "../../model/message";

const useMessageExpanded = () => {
  const [expanded, setExpanded] = useState(false);
  const [message, setMessage] = useState<Message | null>(null);

  const expandMessage = useCallback((message: Message) => {
    setExpanded(true);
    setMessage(message);
  }, []);

  const collapseMessage = useCallback(() => {
    setExpanded(false);
    setMessage(null);
  }, []);

  return { expanded, message, expandMessage, collapseMessage };
};

export default useMessageExpanded;
