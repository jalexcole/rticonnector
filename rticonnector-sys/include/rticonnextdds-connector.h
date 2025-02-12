/*****************************************************************************
 *   (c) 2005-2015 Copyright, Real-Time Innovations.  All rights reserved.    *
 *                                                                            *
 * No duplications, whole or partial, manual or electronic, may be made       *
 * without express written permission.  Any such copies, or revisions thereof,*
 * must display this notice unaltered.                                        *
 * This code contains trade secrets of Real-Time Innovations, Inc.            *
 *                                                                            *
 *****************************************************************************/
// #include "lua_binding/lua_binding_ddsConnector.h"
typedef struct RTIDDSConnector RTI_Connector;

/**
 * Get the number of samples available in the data reader specified by
 * entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 * @param[out] value The number of samples available.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist.
 */
int RTI_Connector_get_sample_count(
    void *self,
    const char *entity_name,
    double *value);

/**
 * Get the value of a JSON field from the info data associated with the
 * specified sample in the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[out] return_value The value of the JSON field. 1 (true) or 0 (false).
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] name The name of the JSON field.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the field does not exist.
 */
int RTI_Connector_get_boolean_from_infos(
    void *self,
    int *return_value,
    const char *entity_name,
    int index,
    const char *name);

/**
 * Set the value of a data writer specified by entity_name from a JSON string.
 *
 * The JSON string must contain a valid JSON object with the same structure as
 * the data type associated with the data writer.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] json The JSON string containing the value to be set.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist or
 *         if the JSON string is invalid.
 */
int RTI_Connector_set_json_instance(
    void *self,
    const char *entity_name,
    const char *json);

/**
 * Set the value of a boolean field in the data writer specified by
 * entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] name The name of the field to be set.
 * @param[in] value The value to be set.  Non-zero values are considered true,
 *                   zero values are considered false.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist,
 *         the field does not exist, or the field is not a boolean.
 */
int RTI_Connector_set_boolean_into_samples(
    void *self,
    const char *entity_name,
    const char *name,
    int value);

/**
 * Set the value of a number field in the data writer specified by
 * entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] name The name of the field to be set.
 * @param[in] value The value to be set.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist,
 *         the field does not exist, or the field is not a number.
 */
int RTI_Connector_set_number_into_samples(
    void *self,
    const char *entity_name,
    const char *name,
    double value);

/**
 * Set the value of a string field in the data writer specified by
 * entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] name The name of the field to be set.
 * @param[in] value The value to be set.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist,
 *         the field does not exist, or the field is not a string.
 */
int RTI_Connector_set_string_into_samples(
    void *self,
    const char *entity_name,
    const char *name,
    const char *value);

/**
 * Get the value of a JSON field from the info data associated with the
 * specified sample in the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] name The name of the JSON field.
 * @param[out] value The value of the JSON field.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the field does not exist.
 */
int RTI_Connector_get_json_from_infos(
    void *self,
    const char *entity_name,
    int index,
    const char *name,
    char **value);

/**
 * Get the JSON representation of the specified sample in the data reader
 * specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[out] json_str The JSON representation of the sample.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the sample does not exist.
 */
int RTI_Connector_get_json_sample(
    void *self,
    const char *entity_name,
    int index,
    char **json_str);

/**
 * Get the JSON representation of the specified member in the specified sample
 * in the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] member_name The name of the member to be retrieved.
 * @param[out] json_str The JSON representation of the member.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the member does not exist.
 */
int RTI_Connector_get_json_member(
    void *self,
    const char *entity_name,
    int index,
    const char *member_name,
    char **json_str);

/**
 * Clear all samples from the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist.
 */
int RTI_Connector_clear(
    void *self,
    const char *entity_name);

/**
 * Read new samples from the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist.
 */
int RTI_Connector_read(
    void *self,
    const char *entity_name);

/**
 * Take new samples from the data reader specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data reader.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist.
 */
int RTI_Connector_take(
    void *self,
    const char *entity_name);

/**
 * Write data to the data writer specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] params_json The JSON string containing the data to be written.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist.
 */
int RTI_Connector_write(
    void *self,
    const char *entity_name,
    const char *params_json);

struct RTI_Connector_Options
{
    /* boolean */ int enable_on_data_event;
    /* boolean */ int one_based_sequence_indexing;
};

#define RTI_Connector_Options_INITIALIZER       \
    {                                           \
        1,    /* enable_on_data_event */        \
            1 /* one_based_sequence_indexing */ \
    }

RTI_Connector *RTI_Connector_new(
    const char *config_name,
    const char *config_file,
    const struct RTI_Connector_Options *options);

void RTI_Connector_delete(RTI_Connector *self);

/**
 * Get the value of a number field from the specified sample in the data reader
 * specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[out] return_value The value of the number field.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] name The name of the field to be retrieved.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the field does not exist.
 */
int RTI_Connector_get_number_from_sample(
    void *self,
    double *return_value,
    const char *entity_name,
    int index,
    const char *name);

/**
 * Get the value of a boolean field from the specified sample in the data reader
 * specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[out] return_value The value of the boolean field.  Non-zero values are
 *                          considered true, zero values are considered false.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] name The name of the field to be retrieved.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the field does not exist.
 */
int RTI_Connector_get_boolean_from_sample(
    void *self,
    int *return_value,
    const char *entity_name,
    int index,
    const char *name);

/**
 * Get the value of a string field from the specified sample in the data reader
 * specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[out] return_value The value of the string field.
 * @param[in] entity_name The name of the data reader.
 * @param[in] index The index of the sample.
 * @param[in] name The name of the field to be retrieved.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist,
 *         the sample index is out of range, or the field does not exist.
 */
int RTI_Connector_get_string_from_sample(
    void *self,
    char **return_value,
    const char *entity_name,
    int index,
    const char *name);

// We will uncomment the following functions when Go wrapper functions are implemented.
/*
int RTI_Connector_get_any_from_sample(
    void *self,
    double *double_value_out,
    RTIBool *bool_value_out,
    char **string_value_out,
    RTI_Connector_AnyValueKind *selected_out,
    const char *entity_name,
    int index,
    const char *name);

int RTI_Connector_get_any_from_info(
    void *self,
    double *double_value_out,
    RTIBool *bool_value_out,
    char **string_value_out,
    RTI_Connector_AnyValueKind *selected_out,
    const char *entity_name,
    int index,
    const char *name);
 */

/**
 * Clear the value of a member in the data writer specified by entity_name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the data writer.
 * @param[in] name The name of the member to be cleared.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist,
 *         the member does not exist, or the member is not a primitive type.
 */
int RTI_Connector_clear_member(
    void *self,
    const char *entity_name,
    const char *name);

/**
 * Returns the DDS_DynamicDataReader associated with the given entity name.
 *
 * \param[in] self The RTI_Connector instance.
 * \param[in] entity_name The name of the entity.
 * \returns The DDS_DynamicDataReader associated with the entity.
 */
void *RTI_Connector_get_datareader( // DDS_DynamicDataReader
    void *self,
    const char *entity_name);

/**
 * Returns the DDS_DynamicDataWriter associated with the given entity name.
 *
 * \param[in] self The RTI_Connector instance.
 * \param[in] entity_name The name of the entity.
 * \returns The DDS_DynamicDataWriter associated with the entity.
 */
void *RTI_Connector_get_datawriter( // DDS_DynamicDataWriter
    void *self,
    const char *entity_name);

/**
 * Returns the native DDS_DynamicData sample associated with the given index
 * for the given entity name.
 *
 * \param[in] self The RTI_Connector instance.
 * \param[in] entity_name The name of the entity.
 * \param[in] index The index of the sample.
 * \returns The native DDS_DynamicData sample associated with the entity.
 */
const void *RTI_Connector_get_native_sample( // DDS_DynamicData
    void *self,
    const char *entity_name,
    int index);

int RTI_Connector_wait_for_data(void *self, int timeout);

/**
 * Waits for data to arrive on the given reader, or for the timeout period to
 * elapse.
 *
 * \param[in] self The RTI_Connector instance.
 * \param[in] ms_timeout The timeout period in milliseconds.  A value of 0
 *                       indicates that the function should not block.
 * \returns 0 on success, or RTI_CONNECTOR_ERROR if an error occurred.
 */
int RTI_Connector_wait_for_data_on_reader(
    void *self,
    int ms_timeout);

/**
 * Waits for the number of acknowledgments for the given writer to change, and
 * returns the new count.
 *
 * \param[in] writer The RTI_Connector data writer instance.
 * \param[in] timeout The maximum time to wait in milliseconds.
 * \returns The new count of acknowledgments, or RTI_CONNECTOR_ERROR if an error
 *          occurred.
 */
int RTI_Connector_wait_for_acknowledgments(
    void *writer,
    int timeout);

/**
 * Waits for the number of matched publications for the given reader to
 * change, and returns the new count.
 *
 * \param[in] reader The RTI_Connector data reader instance.
 * \param[in] ms_timeout The maximum time to wait in milliseconds.
 * \param[out] current_count_change The new number of matched publications.
 *
 * \return 0 on success, or RTI_CONNECTOR_ERROR if the data reader does not exist
 *         or if the wait timed out.
 */
int RTI_Connector_wait_for_matched_publication(
    void *reader,
    int ms_timeout,
    int *current_count_change);



/**
 * Waits for the number of matched subscriptions for the given writer to
 * change, and returns the new count.
 *
 * \param[in] writer The RTI_Connector data writer instance.
 * \param[in] ms_timeout The maximum time to wait in milliseconds.
 * \param[out] current_count_change The new number of matched subscriptions.
 *
 * \return 0 on success, or RTI_CONNECTOR_ERROR if the data writer does not exist
 *         or if the wait timed out.
 */
int RTI_Connector_wait_for_matched_subscription(
    void *writer,
    int ms_timeout,
    int *current_count_change);

/**
 * Gets a JSON string containing the matched subscriptions of the given writer.
 *
 * The JSON string contains a list of subscriptions, where each subscription is a
 * JSON object with the following properties:
 *
 * - "participant_guid": The GUID of the participant that owns the subscription.
 * - "subscription_name": The name of the subscription.
 * - "topic_name": The name of the topic associated with the subscription.
 *
 * @param[in] writer The RTI_Connector instance.
 * @param[out] json_str The JSON string containing the matched subscriptions.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the writer does not exist.
 */
int RTI_Connector_get_matched_subscriptions(
    void *writer,
    char **json_str);

/**
 * Gets a JSON string containing the matched publications of the given reader.
 *
 * The JSON string contains a list of publications, where each publication is a
 * JSON object with the following properties:
 *
 * - "participant_guid": The GUID of the participant that owns the publication.
 * - "publication_name": The name of the publication.
 * - "topic_name": The name of the topic associated with the publication.
 *
 * @param[in] reader The RTI_Connector instance.
 * @param[out] json_str The JSON string containing the matched publications.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the reader does not exist.
 */
int RTI_Connector_get_matched_publications(
    void *reader,
    char **json_str);

char *RTI_Connector_get_last_error_message();

/**
 * Gets the native DDS_DynamicData instance associated with the given entity name.
 *
 * @param[in] self The RTI_Connector instance.
 * @param[in] entity_name The name of the entity.
 * @param[out] native_pointer The native DDS_DynamicData instance associated with the entity.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the entity does not exist.
 */
int RTI_Connector_get_native_instance(
    void *self,
    const char *entity_name,
    const void **native_pointer); // DDS_DynamicData

void RTI_Connector_free_string(char *str);

/**
 * Set the maximum number of objects that can be concurrently accessed by
 * the thread.
 *
 * @param[in] value The maximum number of objects that can be concurrently
 *                  accessed by the thread.
 *
 * @return 0 on success, or RTI_CONNECTOR_ERROR if the value is invalid.
 */
int RTI_Connector_set_max_objects_per_thread(
    int value);