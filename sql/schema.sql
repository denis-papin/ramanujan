prompt Creating sequence PLAYER_ID_SEQ

create sequence player_id_seq
  start with 1
  increment by 1
  minvalue 1
  nocache
  nocycle;

prompt Creating table PLAYER

create table player (
  id number default on null player_id_seq.nextval not null,
  name varchar2(60),
  goals number default 0 not null,
  constraint player_pk primary key (id)
);

prompt Creating sequence AWARD_ID_SEQ

create sequence award_id_seq
  start with 1
  increment by 1
  minvalue 1
  nocache
  nocycle;

prompt Creating table AWARD

create table award (
  id number default on null award_id_seq.nextval not null,
  player_id number not null,
  issuer varchar2(30) not null,
  award_year number(4) not null,
  constraint award_pk primary key (id),
  constraint award_player_fk foreign key (player_id) references player (id),
  constraint award_issuer_ck check (issuer in ('FIFA', 'FRANCE_FOOTBALL')),
  constraint award_year_ck check (award_year between 1980 and 2030)
);

prompt Creating package PLAYER_PKG

CREATE OR REPLACE PACKAGE player_pkg AS
  PROCEDURE increment_goals(
    p_player_id IN player.id%TYPE
  );
END player_pkg;
/

CREATE OR REPLACE PACKAGE BODY player_pkg AS
  c_goals_per_award constant pls_integer := 3;
  c_first_award_year constant pls_integer := 1980;
  c_last_award_year constant pls_integer := 2030;
  c_issuer_fifa constant award.issuer%TYPE := 'FIFA';
  c_issuer_france_football constant award.issuer%TYPE := 'FRANCE_FOOTBALL';

  /* Creates one random award for a player. */
  PROCEDURE create_random_award(
    p_player_id IN player.id%TYPE
  ) AS
    l_issuer award.issuer%type;
    l_award_year award.award_year%type;
  BEGIN
    l_issuer := CASE TRUNC(dbms_random.value(0, 2))
      WHEN 0 THEN c_issuer_fifa
      ELSE c_issuer_france_football
    END;

    l_award_year := TRUNC(dbms_random.value(c_first_award_year, c_last_award_year + 1));

    INSERT INTO award (player_id, issuer, award_year)
    VALUES (p_player_id, l_issuer, l_award_year);
  END create_random_award;

  /* Ensures the player owns one award for every pool of three goals. */
  PROCEDURE create_missing_awards(
    p_player_id IN player.id%TYPE,
    p_current_goals IN player.goals%TYPE
  ) AS
    l_existing_awards pls_integer;
  BEGIN
    SELECT COUNT(*)
      INTO l_existing_awards
      FROM award
     WHERE player_id = p_player_id;

    FOR i IN 1 .. GREATEST(TRUNC(p_current_goals / c_goals_per_award) - l_existing_awards, 0) LOOP
      create_random_award(p_player_id);
    END LOOP;
  END create_missing_awards;

  /* Increments player goals by one and creates any missing awards. */
  PROCEDURE increment_goals(
    p_player_id IN player.id%TYPE
  ) AS
    l_current_goals player.goals%type;
  BEGIN
    IF p_player_id IS NULL THEN
      raise_application_error(-20001, 'p_player_id must not be null');
    END IF;

    UPDATE player
       SET goals = goals + 1
     WHERE id = p_player_id;

    IF SQL%ROWCOUNT = 0 THEN
      raise_application_error(-20003, 'Player not found for id ' || p_player_id);
    END IF;

    SELECT goals
      INTO l_current_goals
      FROM player
     WHERE id = p_player_id;

    create_missing_awards(p_player_id, l_current_goals);
  END increment_goals;
END player_pkg;
/
