import { AudioOutlined, SendOutlined } from "@ant-design/icons";
import { AutoComplete, Button, message, Popover } from "antd";
import { useAtomValue } from "jotai";
import React, { useMemo, useState } from "react";
import {
  useAvailableChannels,
  useCreateSpeakerPhone,
} from "../api/speakerPhone";
import { roundedUserPositionAtom } from "../state/userPosition";

export const SpeakerPhoneButton: React.FC = () => {
  const [messageApi] = message.useMessage();
  const [channelName, setChannelName] = useState("");
  const [isPopoverOpen, setIsPopoverOpen] = useState(false);
  const position = useAtomValue(roundedUserPositionAtom);
  const { trigger } = useCreateSpeakerPhone();
  const [isSending, setIsSending] = useState(false);
  const { data } = useAvailableChannels();
  const availableChannelNames = data ? data.channels : [];

  const [availableChannelNamesFiltered, setAvailableChannelNamesFiltered] =
    React.useState(availableChannelNames);
  const handleSearch = (query: string) => {
    setAvailableChannelNamesFiltered(
      !query
        ? availableChannelNames
        : availableChannelNames.filter((item) => item.includes(query)),
    );
  };
  const options = useMemo(() => {
    return availableChannelNamesFiltered.map((channelName) => ({
      label: channelName,
      value: channelName,
    }));
  }, [availableChannelNamesFiltered]);

  const handleInputChange = (query: string) => {
    setChannelName(query);
  };

  const putSpeakerPhone = async (channelName: string) => {
    if (channelName === "") return;
    if (isSending) return;

    setIsSending(true);
    try {
      await trigger({ position: position ?? undefined, name: channelName });
      setChannelName("");
      setIsPopoverOpen(false);
    } catch (error) {
      console.error("Error creating speakerphone:", error);
      messageApi.error("スピーカーフォンの作成に失敗しました");
    } finally {
      setIsSending(false);
    }
  };
  return (
    <Popover
      content={
        <div>
          <AutoComplete
            className="w-48"
            onSearch={handleSearch}
            placeholder="チャンネル名を入力"
            options={options}
            onChange={handleInputChange}
          />
          <Button
            onClick={() => {
              putSpeakerPhone(channelName);
            }}
            type="text"
            disabled={isSending}
            icon={<SendOutlined />}
          />
        </div>
      }
      trigger="click"
      open={isPopoverOpen}
      onOpenChange={setIsPopoverOpen}
    >
      <Button icon={<AudioOutlined />} />
    </Popover>
  );
};
