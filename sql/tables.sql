-- 
-- user
-- 
DROP TABLE IF EXISTS usr_info ;
CREATE TABLE usr_info (
    id        INT          NOT NULL auto_increment       COMMENT '',

    create_id INT          NOT NULL default 0            COMMENT '',
    created   DATETIME     NOT NULL default '2000-01-01' COMMENT '',
    update_id INT          NOT NULL default 0            COMMENT '',
    updated   DATETIME     NOT NULL default '2000-01-01' COMMENT '',

    uname     VARCHAR(120) NOT NULL default ''           COMMENT '',
    email     VARCHAR(120) NOT NULL default ''           COMMENT '',
    pass      VARCHAR(48)  NOT NULL default ''           COMMENT '',
    salt      VARCHAR(48)  NOT NULL default ''           COMMENT '',
    state     TINYINT      NOT NULL default 0            COMMENT '状态',

    UNIQUE (uname),
    UNIQUE (email),
    PRIMARY KEY                         (id) 
) COMMENT='用户信息' DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci; 

DROP TABLE IF EXISTS usr_login_log;
CREATE TABLE usr_login_log (
    id        INT          NOT NULL auto_increment       COMMENT '',

    create_id INT          NOT NULL default 0            COMMENT '',
    created   DATETIME     NOT NULL default '2000-01-01' COMMENT '',

    ip        VARCHAR(120) NOT NULL default ''           COMMENT '',
    email     VARCHAR(120) NOT NULL default ''           COMMENT '',
    pass      VARCHAR(48)  NOT NULL default ''           COMMENT '',
    salt      VARCHAR(48)  NOT NULL default ''           COMMENT '',
    state     TINYINT      NOT NULL default 0            COMMENT '状态',

    PRIMARY KEY                         (id) 
) COMMENT='用户登录日志' DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci; 

DROP TABLE IF EXISTS usr_dig;
CREATE TABLE usr_dig (
    id        INT          NOT NULL auto_increment       COMMENT '',

    create_id INT          NOT NULL default 0            COMMENT '',
    created   DATETIME     NOT NULL default '2000-01-01' COMMENT '',

    aid       INT          NOT NULL default 0            COMMENT '',
    state     TINYINT      NOT NULL default 0            COMMENT '状态',

    PRIMARY KEY                         (id) 
) COMMENT='用户点赞' DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci; 

-- 
-- tag
-- 
DROP TABLE IF EXISTS tag_info;
CREATE TABLE tag_info (
    id        INT          NOT NULL auto_increment       COMMENT '',

    create_id INT          NOT NULL default 0            COMMENT '',
    created   DATETIME     NOT NULL default '2000-01-01' COMMENT '',
    update_id INT          NOT NULL default 0            COMMENT '',
    updated   DATETIME     NOT NULL default '2000-01-01' COMMENT '',
    views     MEDIUMINT    NOT NULL default 0            COMMENT '',

    pid       INT          NOT NULL default 0            COMMENT '',
    tag       VARCHAR(40)  NOT NULL default ''           COMMENT '',
    atype     TINYINT      NOT NULL default 0            COMMENT '',
    state     TINYINT      NOT NULL default 0            COMMENT '状态',

    UNIQUE                              (tag),
    PRIMARY KEY                         (id) 
) COMMENT='标签' DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci; 

DROP TABLE IF EXISTS tag_topic;
CREATE TABLE tag_topic (
    tid       INT          NOT NULL default 0            COMMENT '',
    aid       INT          NOT NULL default 0            COMMENT '',

    UNIQUE                              (aid, tid),
    PRIMARY KEY                         (tid, aid) 
) COMMENT=''; 

--
-- topic
-- 
DROP TABLE IF EXISTS tpc_info;
CREATE TABLE tpc_info (
    id        INT          NOT NULL auto_increment       COMMENT '',

    create_id     INT          NOT NULL default 0            COMMENT '',
    created       DATETIME     NOT NULL default '2000-01-01' COMMENT '',
    update_id     INT          NOT NULL default 0            COMMENT '',
    updated       DATETIME     NOT NULL default '2000-01-01' COMMENT '',

    views         MEDIUMINT    NOT NULL default 0            COMMENT '',
    count_up      MEDIUMINT    NOT NULL default 0            COMMENT '',
    count_down    MEDIUMINT    NOT NULL default 0            COMMENT '',
    count_reply   MEDIUMINT    NOT NULL default 0            COMMENT '',
    count_comment MEDIUMINT    NOT NULL default 0            COMMENT '',

    atype         TINYINT      NOT NULL default 0            COMMENT '',
    pid           INT          NOT NULL default 0            COMMENT '',
    title         VARCHAR(128) NOT NULL default 0            COMMENT '',
    tag           VARCHAR(128) NOT NULL default 0            COMMENT '',
    content       MEDIUMTEXT   NOT NULL default ''           COMMENT '',

    state         TINYINT      NOT NULL default 0            COMMENT '状态',
    PRIMARY KEY                         (id) 
) COMMENT=''; 

DROP TABLE IF EXISTS tpc_comment;
CREATE TABLE tpc_comment (
    id        INT           NOT NULL auto_increment       COMMENT '',

    create_id INT           NOT NULL default 0            COMMENT '',
    created   DATETIME      NOT NULL default '2000-01-01' COMMENT '',
    update_id INT           NOT NULL default 0            COMMENT '',
    updated   DATETIME      NOT NULL default '2000-01-01' COMMENT '',

    up        MEDIUMINT     NOT NULL default 0            COMMENT '',
    down      MEDIUMINT     NOT NULL default 0            COMMENT '',

    pid       INT           NOT NULL default 0            COMMENT '',
    atype     TINYINT       NOT NULL default 0            COMMENT '',
    content	  VARCHAR(3000) NOT NULL default ''	          COMMENT '',

    state     TINYINT       NOT NULL default 0            COMMENT '状态',
    PRIMARY KEY                         (id) 
) COMMENT=''; 

