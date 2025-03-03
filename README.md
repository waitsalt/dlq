# Deal Life Question

## run
> pre: you need install rust toolchain before run.
1. download code:
```git
git clone https://github.com/waitsalt/dlq
```

2. run code:
```
cd dlp
cargo run
```

## api
### client
#### post
- /v0/posts
    - request:
        - method: get
        - parameter:
            - page: u16
            - limit: u16
            - keyword: String
        - header: null
        - body: null
        - desc: 获取符合要求的帖子, limit为返回帖子数量, 默认为20; page为返回第几页帖子(从零开始), keyword 为搜索帖子的关键词
    - response:
        - status code: 200  
          body:  
            - message: success  
            - data: PostListResponse
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
          
- /v0/posts
    - request:
        - method: post
        - parameter: null
        - header: 
            - Authorization: Bearer <access_token>
        - body: PostCreatePayload
        - desc: 创建一个新的帖子
    - response:
        - status code: 200  
          body:  
            - message: success  
            - data: Post
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null

- /v0/posts/{post_id: String}
    - request:
        - method: get
        - parameter: null
        - header: null
        - body: null
        - desc: 获取帖子
    - response:
        - status code: 200  
          body: 
            - message: success  
            - data: Post
        - status code: 404  
          body: 
            - message: not found
            - data: null

- /v0/posts/{post_id: String}
    - request:
        - method: delete
        - parameter: null
        - body: null
        - header: 
            - Authorization: Bearer <access_token>
        - desc: 删除一个用户发布的帖子
    - response:
        - status code: 204  
          body:  
            - message: success  
            - data: null 
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
        - status code: 403  
          body:  
            - message: not the poster of the post
            - data: null

- /v0/posts/{post_id: String}
    - request:
        - method: patch
        - parameter: null
        - body: PostUpdatePayload
        - header: 
            - Authorization: Bearer <access_token>
        - desc: 更新一个用户发布的帖子
    - response:
        - status code: 200  
          body:
            - message: success  
            - data: Post
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
        - status code: 404  
          body:  
            - message: not found
            - data: null

#### user
- /v0/users
    - request:
        - method: get
        - parameter:
            - page: u16
            - limit: u16
            - keyword: String
        - header: null
        - body: null
        - desc: 获取符合要求的用户, limit为返回用户数量, 默认为20; page为返回第几页用户(从零开始), keyword 为搜索用户的关键词
    - response:
        - status code: 200  
          body:  
            - message: success  
            - data: UserPublicListResponse
          
- /v0/users
    - request:
        - method: post
        - parameter: null
        - body: UserCreatePayload
        - header: null
        - desc: 创建一个新的用户
    - response:
        - status code: 200  
          body:  
            - message: success  
            - data:  UserPublic
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null

- /v0/users/{user_id: String}
    - request:
        - method: get
        - parameter: null
        - body: null
        - header: null
        - desc: 获取用户信息
    - response:
        - status code: 200  
          body: 
            - message: success  
            - data: UserPublic
        - status code: 401  
          body: 
            - message: unauthorized
            - data: null
        - status code: 404  
          body:  
            - message: not find user
            - data: null

- /v0/users/{user_id: String}
    - request:
        - method: delete
        - parameter: null
        - body: null
        - header: 
            - Authorization: Bearer <access_token>
        - desc: 删除用户
    - response:
        - status code: 204  
          body:  
            - message: success  
            - data: null 
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
        - status code: 404  
          body:  
            - message: not find user
            - data: null

- /v0/users/{user_id: String}
    - request:
        - method: patch
        - parameter: null
        - body: UserUpdatePayload
        - header: 
            - Authorization: Bearer <access_token>
        - desc: 更新用户的个人信息
    - response:
        - status code: 200  
          body:
            - message: success  
            - data: UserPublic
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
        - status code: 404  
          body:  
            - message: not find user
            - data: null

- /v0/users/{user_id: String}/posts
    - request:
        - method: get
        - parameter:
            - page: u16
            - limit: u16
        - body: null
        - header: 
            - Authorization: Bearer <access_token>
        - desc: 更新用户的帖子
    - response:
        - status code: 200  
          body:
            - message: success  
            - data: UserPublicListResponse
        - status code: 401  
          body:  
            - message: unauthorized
            - data: null
        - status code: 404  
          body:  
            - message: not find user
            - data: null:q