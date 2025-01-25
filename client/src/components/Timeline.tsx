import { Avatar, Divider, List, Skeleton } from "antd";
import React, { useCallback, useEffect, useState } from "react";
import InfiniteScroll from "react-infinite-scroll-component";
import { InputMessage } from "./InputMessage";

interface DataType {
  gender: string;
  name: {
    title: string;
    first: string;
    last: string;
  };
  email: string;
  picture: {
    large: string;
    medium: string;
    thumbnail: string;
  };
  nat: string;
}

export const Timeline: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<DataType[]>([]);

  const loadMoreData = useCallback(() => {
    if (loading) {
      return;
    }
    setLoading(true);
    fetch(
      "https://randomuser.me/api/?results=10&inc=name,gender,email,nat,picture&noinfo",
    )
      .then((res) => res.json())
      .then((body) => {
        setData([...data, ...body.results]);
        setLoading(false);
      })
      .catch(() => {
        setLoading(false);
      });
  }, [data, loading]);

  useEffect(() => {
    loadMoreData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="flex flex-col h-screen w-full bg-background-primary">
      <div id="scrollableDiv" className="size-full overflow-auto px-4">
        <InfiniteScroll
          dataLength={data.length}
          next={loadMoreData}
          hasMore={data.length < 50}
          loader={<Skeleton avatar paragraph={{ rows: 1 }} active />}
          endMessage={<Divider plain>It is all, nothing more</Divider>}
          scrollableTarget="scrollableDiv"
        >
          <List
            dataSource={data}
            renderItem={(item) => (
              // TODO: traQ ID に置き換える
              <List.Item key={item.email}>
                <List.Item.Meta
                  avatar={<Avatar src={item.picture.large} />} // TODO: traQ のアイコンに置き換える
                  title={item.name.last} // TODO: traQ のユーザー名に置き換える
                  description={item.email} // TODO: traQ ID に置き換える
                />
                Content Content Content Content Content Content Content Content
                Content Content Content Content Content Content Content Content
                Content Content Content Content Content Content Content
              </List.Item>
            )}
          />
        </InfiniteScroll>
      </div>
      <InputMessage />
    </div>
  );
};
