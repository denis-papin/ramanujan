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
  points number default 0 not null,
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
  constraint award_issuer_ck check (issuer in ('GOLD', 'SILVER')),
  constraint award_year_ck check (award_year between 1980 and 2030)
);

prompt Creating package PLAYER_PKG

CREATE OR REPLACE PACKAGE player_pkg AS
  PROCEDURE increment_points(
    p_player_id IN player.id%TYPE
  );
  PROCEDURE decrease_points(
    p_player_id IN player.id%TYPE
  );
END player_pkg;
/

CREATE OR REPLACE PACKAGE BODY player_pkg AS
  c_points_per_award constant pls_integer := 3;
  c_first_award_year constant pls_integer := 1980;
  c_last_award_year constant pls_integer := 2030;
  c_issuer_gold constant award.issuer%TYPE := 'GOLD';
  c_issuer_silver constant award.issuer%TYPE := 'SILVER';

  /* Creates one random award for a player. */
  PROCEDURE create_random_award(
    p_player_id IN player.id%TYPE
  ) AS
    l_issuer award.issuer%type;
    l_award_year award.award_year%type;
  BEGIN
    l_issuer := CASE TRUNC(dbms_random.value(0, 2))
      WHEN 0 THEN c_issuer_gold
      ELSE c_issuer_silver
    END;

    l_award_year := TRUNC(dbms_random.value(c_first_award_year, c_last_award_year + 1));

    INSERT INTO award (player_id, issuer, award_year)
    VALUES (p_player_id, l_issuer, l_award_year);
  END create_random_award;

  /* Deletes the newest award for a player. */
  PROCEDURE delete_latest_award(
    p_player_id IN player.id%TYPE,
    p_deleted_count OUT pls_integer
  ) AS
  BEGIN
    DELETE FROM award
     WHERE id = (
       SELECT id
         FROM (
           SELECT id
             FROM award
            WHERE player_id = p_player_id
            ORDER BY id DESC
         )
        WHERE ROWNUM = 1
     );

    p_deleted_count := SQL%ROWCOUNT;
  END delete_latest_award;

  /* Reconciles award count with the current number of points. */
  PROCEDURE reconcile_awards(
    p_player_id IN player.id%TYPE,
    p_current_points IN player.points%TYPE
  ) AS
    l_existing_awards pls_integer;
    l_expected_awards pls_integer;
    l_deleted_count pls_integer;
  BEGIN
    SELECT COUNT(*)
      INTO l_existing_awards
      FROM award
     WHERE player_id = p_player_id;

    l_expected_awards := TRUNC(p_current_points / c_points_per_award);

    FOR i IN 1 .. GREATEST(l_expected_awards - l_existing_awards, 0) LOOP
      create_random_award(p_player_id);
    END LOOP;

    FOR i IN 1 .. GREATEST(l_existing_awards - l_expected_awards, 0) LOOP
      delete_latest_award(p_player_id, l_deleted_count);
    END LOOP;
  END reconcile_awards;

  /* Increments player points by one and creates any missing awards. */
  PROCEDURE increment_points(
    p_player_id IN player.id%TYPE
  ) AS
    l_current_points player.points%TYPE;
  BEGIN
    IF p_player_id IS NULL THEN
      raise_application_error(-20001, 'p_player_id must not be null');
    END IF;

    UPDATE player
       SET points = points + 1
     WHERE id = p_player_id;

    IF SQL%ROWCOUNT = 0 THEN
      raise_application_error(-20003, 'Player not found for id ' || p_player_id);
    END IF;

    SELECT points
      INTO l_current_points
      FROM player
     WHERE id = p_player_id;

    reconcile_awards(p_player_id, l_current_points);
  END increment_points;

  /* Decrements player points by one and removes any extra awards. */
  PROCEDURE decrease_points(
    p_player_id IN player.id%TYPE
  ) AS
    l_current_points player.points%TYPE;
  BEGIN
    IF p_player_id IS NULL THEN
      raise_application_error(-20001, 'p_player_id must not be null');
    END IF;

    SELECT points
      INTO l_current_points
      FROM player
     WHERE id = p_player_id;

    IF l_current_points <= 0 THEN
      raise_application_error(-20004, 'Player points cannot be negative for id ' || p_player_id);
    END IF;

    UPDATE player
       SET points = points - 1
     WHERE id = p_player_id;

    SELECT points
      INTO l_current_points
      FROM player
     WHERE id = p_player_id;

    reconcile_awards(p_player_id, l_current_points);
  EXCEPTION
    WHEN no_data_found THEN
      raise_application_error(-20003, 'Player not found for id ' || p_player_id);
  END decrease_points;
END player_pkg;
/
