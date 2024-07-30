// @generated automatically by Diesel CLI.

diesel::table! {
    t_artful_question (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        artful_id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_artful_question_rel (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        scene_code -> Nullable<Text>,
        artful_id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_chapter (_id) {
        _id -> Integer,
        title -> Text,
        chapter -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    t_chapter_question (_id) {
        _id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
        chapter_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_deleted (_id) {
        _id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_dictionary (_id) {
        _id -> Nullable<Integer>,
        name -> Nullable<Text>,
        value -> Nullable<Binary>,
        groups -> Nullable<Text>,
    }
}

diesel::table! {
    t_district_exam_regular (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
    }
}

diesel::table! {
    t_district_exam_rule (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        areacode -> Nullable<Integer>,
        total_score -> Nullable<Integer>,
        pass_score -> Nullable<Integer>,
        time -> Nullable<Integer>,
        question_count -> Nullable<Integer>,
        judge_score -> Nullable<Integer>,
        single_score -> Nullable<Integer>,
        multi_score -> Nullable<Integer>,
    }
}

diesel::table! {
    t_exam_regular (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exam_regular_scene (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exam_regular_scene_rel (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exam_rule (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        areacode -> Nullable<Integer>,
        total_score -> Nullable<Integer>,
        pass_score -> Nullable<Integer>,
        time -> Nullable<Integer>,
        question_count -> Nullable<Integer>,
        judge_score -> Nullable<Integer>,
        single_score -> Nullable<Integer>,
        multi_score -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exam_rule_scene (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        areacode -> Nullable<Integer>,
        total_score -> Nullable<Integer>,
        pass_score -> Nullable<Integer>,
        time -> Nullable<Integer>,
        question_count -> Nullable<Integer>,
        judge_score -> Nullable<Integer>,
        single_score -> Nullable<Integer>,
        multi_score -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exchange_exam_regular (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exchange_exam_regular_scene (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exchange_exam_regular_scene_rel (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        chapter_id -> Nullable<Integer>,
        count1 -> Nullable<Integer>,
        count2 -> Nullable<Integer>,
        count3 -> Nullable<Integer>,
        count4 -> Nullable<Integer>,
        count5 -> Nullable<Integer>,
        count6 -> Nullable<Integer>,
        count7 -> Nullable<Integer>,
        count -> Nullable<Integer>,
        areacode -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exchange_exam_rule (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        areacode -> Nullable<Integer>,
        total_score -> Nullable<Integer>,
        pass_score -> Nullable<Integer>,
        time -> Nullable<Integer>,
        question_count -> Nullable<Integer>,
        judge_score -> Nullable<Integer>,
        single_score -> Nullable<Integer>,
        multi_score -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_exchange_exam_rule_scene (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        areacode -> Nullable<Integer>,
        total_score -> Nullable<Integer>,
        pass_score -> Nullable<Integer>,
        time -> Nullable<Integer>,
        question_count -> Nullable<Integer>,
        judge_score -> Nullable<Integer>,
        single_score -> Nullable<Integer>,
        multi_score -> Nullable<Integer>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_fallible_question (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        question_id -> Nullable<Binary>,
    }
}

diesel::table! {
    t_fallible_question_scene (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        question_id -> Nullable<Binary>,
        scene_code -> Nullable<Text>,
    }
}

diesel::table! {
    t_knowledge_summary (_id) {
        _id -> Nullable<Integer>,
        summary -> Nullable<Text>,
        image -> Nullable<Text>,
        knowledge_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_media (_id) {
        _id -> Nullable<Integer>,
        media_key -> Nullable<Text>,
        media_content -> Nullable<Binary>,
        media_url -> Nullable<Text>,
        hd_image_url -> Nullable<Text>,
        practice_hd_image_url -> Nullable<Text>,
    }
}

diesel::table! {
    t_question (_id) {
        _id -> Integer,
        question_id -> Integer,
        media_type -> Nullable<Integer>,
        chapter_id -> Integer,
        label -> Nullable<Text>,
        question -> Binary,
        media_key -> Nullable<Text>,
        answer -> Integer,
        option_a -> Nullable<Text>,
        option_b -> Nullable<Text>,
        option_c -> Nullable<Text>,
        option_d -> Nullable<Text>,
        option_e -> Nullable<Text>,
        option_f -> Nullable<Text>,
        option_g -> Nullable<Text>,
        option_h -> Nullable<Text>,
        explain -> Nullable<Binary>,
        concise_explain -> Nullable<Binary>,
        keywords -> Nullable<Binary>,
        assured_keywords -> Nullable<Binary>,
        illiteracy_explain -> Nullable<Binary>,
        illiteracy_keywords -> Nullable<Binary>,
        difficulty -> Integer,
        wrong_rate -> Double,
        option_type -> Integer,
        knack_keyword -> Nullable<Binary>,
        knack_img_url -> Nullable<Binary>,
        knack_detail -> Nullable<Binary>,
        knack_voice_txt -> Nullable<Binary>,
        m -> Integer,
        sort -> Nullable<Integer>,
        supreme -> Nullable<Integer>,
        explain_keywords -> Nullable<Binary>,
    }
}

diesel::table! {
    t_question_knowledge (_id) {
        _id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
        knowledge_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_question_label (_id) {
        _id -> Nullable<Integer>,
        source_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        label_type -> Nullable<Text>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    t_question_label_rel (_id) {
        _id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
        label_id -> Nullable<Integer>,
        label_type -> Nullable<Text>,
    }
}

diesel::table! {
    t_question_sort (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        course -> Nullable<Text>,
        question_id -> Nullable<Integer>,
        sort -> Nullable<Integer>,
    }
}

diesel::table! {
    t_tag (_id) {
        _id -> Nullable<Integer>,
        question_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
        chapter_id -> Nullable<Integer>,
    }
}

diesel::table! {
    t_version (_id) {
        _id -> Nullable<Integer>,
        major_version -> Nullable<Integer>,
        version -> Nullable<Integer>,
    }
}

diesel::table! {
    t_vp (_id) {
        _id -> Nullable<Integer>,
        car_type -> Nullable<Text>,
        kemu -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        city -> Nullable<Integer>,
        tiku_version -> Nullable<Integer>,
        district -> Nullable<Integer>,
        scene_code -> Nullable<Integer>,
        pattern_code -> Nullable<Integer>,
        list -> Nullable<Binary>,
    }
}

diesel::joinable!(t_district_exam_regular -> t_chapter (chapter_id));
diesel::joinable!(t_exam_regular -> t_chapter (chapter_id));
diesel::joinable!(t_exam_regular_scene -> t_chapter (chapter_id));
diesel::joinable!(t_exam_regular_scene_rel -> t_chapter (chapter_id));
diesel::joinable!(t_exchange_exam_regular -> t_chapter (chapter_id));
diesel::joinable!(t_exchange_exam_regular_scene -> t_chapter (chapter_id));
diesel::joinable!(t_exchange_exam_regular_scene_rel -> t_chapter (chapter_id));

diesel::allow_tables_to_appear_in_same_query!(
    t_artful_question,
    t_artful_question_rel,
    t_chapter,
    t_chapter_question,
    t_deleted,
    t_dictionary,
    t_district_exam_regular,
    t_district_exam_rule,
    t_exam_regular,
    t_exam_regular_scene,
    t_exam_regular_scene_rel,
    t_exam_rule,
    t_exam_rule_scene,
    t_exchange_exam_regular,
    t_exchange_exam_regular_scene,
    t_exchange_exam_regular_scene_rel,
    t_exchange_exam_rule,
    t_exchange_exam_rule_scene,
    t_fallible_question,
    t_fallible_question_scene,
    t_knowledge_summary,
    t_media,
    t_question,
    t_question_knowledge,
    t_question_label,
    t_question_label_rel,
    t_question_sort,
    t_tag,
    t_version,
    t_vp,
);
